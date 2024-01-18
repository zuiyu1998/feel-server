use std::sync::Arc;

use crate::{
    config::load_config,
    helper::{EncryptHelper, JwtHelper},
    state::{self},
    Config, ServerResult,
};
use poem::{listener::TcpListener, middleware::Tracing, Endpoint, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use tracing::Level;

mod apis;
mod middleware;
mod response;

pub mod security;

pub fn build_app(app: impl Endpoint, config: &Arc<Config>) -> impl Endpoint {
    let jwt_helper = JwtHelper::from_config(&config.jwt);
    let encrypt_helper = EncryptHelper::new(config.encrypt.secure.as_bytes());

    app.with(Tracing).data(jwt_helper).data(encrypt_helper)
}

pub async fn start_web() -> ServerResult<()> {
    let config = load_config()?;

    let config: Arc<Config> = Arc::new(config);

    tracing_subscriber::fmt::fmt()
        .with_max_level(Level::INFO)
        .init();

    let state = state::initialize(&config).await?;

    let api_service = OpenApiService::new(
        apis::create_apis(),
        &config.system.name,
        &config.system.version,
    )
    .server(config.server.get_api_url());

    tracing::info!("swagger_ui url : {}", config.server.get_swigger_url());

    let ui = api_service.swagger_ui();

    let app = Route::new()
        .nest("/api", api_service)
        .nest("/", ui)
        .data(state);

    let app = build_app(app, &config);

    Server::new(TcpListener::bind(config.server.get_addr()))
        .run(app)
        .await?;

    Ok(())
}
