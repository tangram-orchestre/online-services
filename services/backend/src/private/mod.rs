use poem_openapi::{OpenApi, Tags};

mod error;

mod misc;
mod semesters;
mod users;

pub(super) fn api() -> impl OpenApi {
    (misc::Api, semesters::Api, users::Api)
}

#[derive(Tags)]
enum PrivateApiTags {
    User,
    Semesters,
    Misc,
}
