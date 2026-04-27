use poem_openapi::{OpenApi, Tags};

mod concerts;
mod locations;
mod misc;
mod pieces;
mod semesters;
mod users;

pub(super) fn api() -> impl OpenApi {
    (
        misc::Api,
        semesters::Api,
        users::Api,
        concerts::Api,
        locations::Api,
        pieces::Api,
    )
}

#[derive(Tags)]
enum PrivateApiTags {
    User,
    Semesters,
    Misc,
    Concerts,
    Locations,
    Pieces,
}
