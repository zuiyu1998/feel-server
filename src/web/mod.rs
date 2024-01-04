use crate::{config::load_config, ServerResult};
use poem::{listener::TcpListener, Route, Server};
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
    tracing_subscriber::fmt::init();

    let api_service = OpenApiService::new(Api, config.system.name, config.system.version)
        .server(config.server.get_api_url());

    tracing::info!("swagger_ui url : {}", config.server.get_swigger_url());

    let ui = api_service.swagger_ui();

    Server::new(TcpListener::bind(config.server.get_addr()))
        .run(Route::new().nest("/api", api_service).nest("/", ui))
        .await?;

    Ok(())
}
