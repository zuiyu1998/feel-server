use poem_openapi::{OpenApi, Tags};

mod label_api;
mod user_api;

pub fn create_apis() -> impl OpenApi {
    (user_api::UserApi, label_api::LabelApi)
}

#[derive(Tags)]
enum ApiTags {
    UserApi,
    LabelApi,
}
