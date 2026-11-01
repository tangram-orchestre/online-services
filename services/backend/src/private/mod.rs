use poem_openapi::{OpenApi, Tags};

mod concerts;
mod images;
mod locations;
mod misc;
mod people;
mod pieces;
mod semesters;
mod settings;
mod users;

pub(super) fn api() -> impl OpenApi {
    (
        misc::Api,
        semesters::Api,
        users::Api,
        concerts::Api,
        locations::Api,
        pieces::Api,
        people::Api,
        images::Api,
        settings::Api,
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
    People,
    Images,
    Settings,
}
