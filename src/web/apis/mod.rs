use super::middleware::PermissionMiddleware;
use poem::{Endpoint, EndpointExt};
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

pub fn common_middleware(ep: impl Endpoint) -> impl Endpoint {
    ep.with(PermissionMiddleware::default())
}
