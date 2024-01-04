use feel_server::web::start_web;

#[tokio::main]

async fn main() {
    if let Err(e) = start_web().await {
        println!("start web error: {}", e);
    }
}
