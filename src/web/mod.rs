use std::sync::Arc;

use crate::{
    config::load_config,
    state::{self},
    Config, ServerResult,
};
use poem::{listener::TcpListener, middleware::Tracing, EndpointExt, Route, Server};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {name}!")),
            None => PlainText("hello!".to_string()),
        }
    }
}

pub async fn start_web() -> ServerResult<()> {
    let config = load_config()?;

    let config: Arc<Config> = Arc::new(config);

    tracing_subscriber::fmt::init();

    let state = state::initialize(&config).await?;

    let api_service = OpenApiService::new(Api, &config.system.name, &config.system.version)
        .server(config.server.get_api_url());

    tracing::info!("swagger_ui url : {}", config.server.get_swigger_url());

    let ui = api_service.swagger_ui();

    let app = Route::new()
        .nest("/api", api_service)
        .nest("/", ui)
        .data(state)
        .with(Tracing);

    Server::new(TcpListener::bind(config.server.get_addr()))
        .run(app)
        .await?;

    Ok(())
}
