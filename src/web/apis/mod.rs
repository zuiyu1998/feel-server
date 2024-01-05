use poem_openapi::{OpenApi, Tags};

mod user_api;

mod response;

pub fn create_apis() -> impl OpenApi {
    user_api::Api
}

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    UserApi,
}
