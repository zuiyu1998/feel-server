use std::sync::Arc;

use crate::{
    config::load_config,
    state::{self},
    Config, ServerResult,
};
use poem::{listener::TcpListener, middleware::Tracing, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use tracing::Level;

mod apis;

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
        .with(Tracing)
        .data(state);

    Server::new(TcpListener::bind(config.server.get_addr()))
        .run(app)
        .await?;

    Ok(())
}
