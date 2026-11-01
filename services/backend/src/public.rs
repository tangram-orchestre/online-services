use altcha_lib_rs::{Challenge, ChallengeOptions, verify_solution};
use chrono::{NaiveDate, NaiveTime, Utc};
use diesel::{
    ExpressionMethods, NullableExpressionMethods, OptionalExtension, QueryDsl, Queryable,
};
use diesel_async::RunQueryDsl;
use lettre::{
    AsyncTransport, Message,
    message::{Mailbox, header::ContentType},
};
use poem::{
    IntoResponse, Request, Response, handler,
    http::{StatusCode, header},
    web::{Data, Path},
};
use poem_openapi::{
    ApiResponse, Object, OpenApi, Tags,
    payload::{Json, PlainText},
    types::{Any, Email},
};

use crate::AppState;
use crate::error::Result;
use crate::schema::{concerts, images, locations, rooms, settings};

diesel::define_sql_function!(
    /// The first of `a` and `b` that is not null.
    fn coalesce(a: diesel::sql_types::Nullable<diesel::sql_types::Uuid>, b: diesel::sql_types::Nullable<diesel::sql_types::Uuid>) -> diesel::sql_types::Nullable<diesel::sql_types::Uuid>
);

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
    Concerts,
}

/// A concert as shown on the showcase website (without its programme).
#[derive(Debug, Object, Queryable)]
struct PublicConcert {
    id: i32,
    date: NaiveDate,
    /// Optional name, e.g. « Concert de Noël ».
    name: Option<String>,
    /// Name of the venue, absent while not chosen.
    location_name: Option<String>,
    /// City of the venue, absent while not chosen.
    city: Option<String>,
    starts_at: Option<NaiveTime>,
    tickets_url: Option<String>,
    video_url: Option<String>,
    /// Image of the concert, else photo of its room, else of its venue, else
    /// the default image: served at
    /// `/public/images/{image_id}`.
    image_id: Option<uuid::Uuid>,
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
    /// Get the public concerts, most recent first
    #[oai(path = "/concerts", method = "get", tag = PublicApiTags::Concerts)]
    async fn list_public_concerts(
        &self,
        state: Data<&AppState>,
    ) -> Result<Json<Vec<PublicConcert>>> {
        let mut conn = state.db_connection_pool.get().await.unwrap();
        Ok(Json(
            concerts::table
                .left_join(rooms::table.inner_join(locations::table))
                .filter(concerts::public.eq(true))
                .order(concerts::date.desc())
                .select((
                    concerts::id,
                    concerts::date,
                    concerts::name,
                    locations::name.nullable(),
                    locations::city.nullable(),
                    concerts::starts_at,
                    concerts::tickets_url,
                    concerts::video_url,
                    // The most specific image: concert, room, venue, then default.
                    coalesce(
                        concerts::image_id,
                        coalesce(
                            rooms::image_id.nullable(),
                            coalesce(
                                locations::image_id.nullable(),
                                settings::table
                                    .select(settings::default_concert_image_id)
                                    .single_value(),
                            ),
                        ),
                    ),
                ))
                .load::<PublicConcert>(&mut conn)
                .await?,
        ))
    }

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

/// Health probe used by Docker healthchecks (not part of the OpenAPI spec):
/// the backend is healthy when it can reach its database.
#[handler]
pub async fn health(state: Data<&AppState>) -> StatusCode {
    let Ok(mut conn) = state.get_db_connection().await else {
        return StatusCode::SERVICE_UNAVAILABLE;
    };

    match diesel::sql_query("SELECT 1").execute(&mut conn).await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            tracing::error!("Health check query failed: {:?}", e);
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
}

/// An uploaded image. Images never change (a new image gets a new id), so they
/// can be cached forever.
#[handler]
pub async fn image(
    Path(image_id): Path<String>,
    state: Data<&AppState>,
    req: &Request,
) -> Response {
    let Ok(id) = uuid::Uuid::parse_str(&image_id) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let etag = format!("\"{id}\"");
    if req
        .headers()
        .get(header::IF_NONE_MATCH)
        .is_some_and(|value| value.as_bytes() == etag.as_bytes())
    {
        return StatusCode::NOT_MODIFIED.into_response();
    }

    let Ok(mut conn) = state.get_db_connection().await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    match images::table
        .find(id)
        .select((images::content_type, images::data))
        .first::<(String, Vec<u8>)>(&mut conn)
        .await
        .optional()
    {
        Ok(Some((content_type, data))) => Response::builder()
            .content_type(content_type)
            .header(header::CACHE_CONTROL, "public, max-age=31536000, immutable")
            .header(header::ETAG, etag)
            .body(data),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => {
            tracing::error!("Could not load image {id}: {err}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
