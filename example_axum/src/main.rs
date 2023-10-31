use std::error::Error;

use axum::Router;
use swaggapi::handler::RouterExt;
use tokio::net::TcpListener;

#[swaggapi::operation(Get "" "/index")]
pub async fn index() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new().routes(&[&index]);

    let listener = TcpListener::bind("127.0.0.1:1337").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
