use altcha_lib_rs::{Challenge, ChallengeOptions, verify_solution};
use chrono::Utc;
use lettre::{
    AsyncTransport, Message,
    message::{Mailbox, header::ContentType},
};
use poem::web::Data;
use poem_openapi::{
    ApiResponse, Object, OpenApi, Tags,
    payload::{Json, PlainText},
    types::{Any, Email},
};

use crate::AppState;

pub struct PublicApi;

#[derive(Debug, Clone, Object)]
struct ContactForm {
    /// Name of the sender
    #[oai(validator { min_length = 2,  max_length = 50 })]
    name: String,
    /// Email name of the sender
    #[oai(validator { max_length = 50 })]
    email: Email,
    /// Subject of the message
    #[oai(validator { min_length = 2,  max_length = 50 })]
    subject: String,
    /// Message to be sent
    #[oai(validator { min_length = 3, max_length = 2000 })]
    message: String,
    /// Altcha captcha
    altcha: Any<altcha_lib_rs::Payload>,
}

#[derive(Tags)]
enum PublicApiTags {
    Contact,
}

#[derive(ApiResponse)]
enum SendContactFormResponse {
    #[oai(status = 200)]
    Success,
    #[oai(status = 400)]
    BadRequest(PlainText<String>),
}

const ALTCHA_EXPIRATION_MINUTES: i64 = 1;
// See https://altcha.org/docs/complexity/
const ALTCHA_CHALLENGE_COMPLEXITY: u64 = 100_000;

#[OpenApi]
impl PublicApi {
    /// Retrieve an Altcha challenge that can be used to validate a contact form
    #[oai(path = "/altcha_challenge", method = "get", tag = PublicApiTags::Contact)]
    async fn altcha_challenge(&self, state: Data<&AppState>) -> Json<Any<Challenge>> {
        let challenge = altcha_lib_rs::create_challenge(ChallengeOptions {
            hmac_key: &state.altcha_secret,
            expires: Some(Utc::now() + chrono::TimeDelta::minutes(ALTCHA_EXPIRATION_MINUTES)),
            max_number: Some(ALTCHA_CHALLENGE_COMPLEXITY),
            ..Default::default()
        })
        .expect("should be ok");

        tracing::debug!("Altcha challenge created: {:?}", challenge);

        Json(Any(challenge))
    }

    /// Send a contact form
    #[oai(path = "/send_contact_form", method = "post", tag = PublicApiTags::Contact)]
    async fn send_contact_form(
        &self,
        contact_form: Json<ContactForm>,
        state: Data<&AppState>,
    ) -> SendContactFormResponse {
        if let Err(e) = verify_solution(&contact_form.altcha.0, &state.altcha_secret, true) {
            tracing::error!("Altcha challenge could not be validated: {:?}", e);
            return SendContactFormResponse::BadRequest(PlainText(
                "Altcha challenge could not be validated".to_string(),
            ));
        }

        // Protect against replay attacks
        {
            let mut validated_challenges = state.altcha_validated_challenges.lock().unwrap();
            if validated_challenges.contains_key(&contact_form.altcha.0.salt) {
                tracing::warn!(
                    "Altcha challenge already validated {:?}",
                    contact_form.email
                );
                return SendContactFormResponse::BadRequest(PlainText(
                    "Altcha challenge already validated".to_string(),
                ));
            }

            // Remove expired challenges
            let now = Utc::now();
            validated_challenges.retain(|_, datetime| {
                now - *datetime > chrono::TimeDelta::minutes(ALTCHA_EXPIRATION_MINUTES)
            });

            validated_challenges.insert(contact_form.altcha.0.salt.clone(), now);
        }

        let email = match Message::builder()
            .from(
                "NoReply Tangram <noreply@tangram-orchestre.fr>"
                    .parse()
                    .unwrap(),
            )
            .reply_to(Mailbox::new(
                Some(contact_form.name.clone()),
                contact_form.email.0.parse().unwrap(),
            ))
            .to("Contact Tangram <contact@tangram-orchestre.fr>"
                .parse()
                .unwrap())
            .subject(format!(
                "[Website Contact Form] {} - {}",
                contact_form.name, contact_form.subject
            ))
            .header(ContentType::TEXT_PLAIN)
            .body(contact_form.message.clone())
        {
            Ok(email) => email,
            e => {
                tracing::error!("Could not create email: {:?}", e);
                return SendContactFormResponse::BadRequest(PlainText(
                    "Could not create email".to_string(),
                ));
            }
        };

        // Send the email
        match state.mailer.send(email).await {
            Ok(_) => {
                tracing::info!(
                    "Email sent (sender: {:?}), alcha time: {:?}ms",
                    contact_form.email,
                    contact_form.altcha.0.took.unwrap_or(0)
                );
                SendContactFormResponse::Success
            }
            Err(_e) => {
                tracing::error!("Could not send email: {:?}", _e);
                SendContactFormResponse::BadRequest(PlainText("Could not send email".to_string()))
            }
        }
    }
}
