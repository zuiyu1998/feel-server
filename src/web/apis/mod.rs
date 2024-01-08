use poem_openapi::{OpenApi, Tags};

mod user_api;

pub fn create_apis() -> impl OpenApi {
    user_api::UserApi
}

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    UserApi,
}
