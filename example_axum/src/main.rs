use std::error::Error;

use axum::{Json, Router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use swaggapi::handler::RouterExt;
use tokio::net::TcpListener;

#[derive(Deserialize, JsonSchema)]
pub struct SubmitForm {}

#[swaggapi::get("", "/index")]
pub async fn index() -> &'static str {
    "Hello world"
}

/// here be dragons
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct JsonBody {
    foo: i32,
    bar: String,
    baz: Vec<u8>,
}
pub type JsonResponse = JsonBody;

/// Huiii
///
/// wow some explanation
#[swaggapi::post("", "/json")]
pub async fn json(_json: Json<JsonBody>) -> Json<JsonResponse> {
    todo!()
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new().routes(&[&index, &json]);

    let listener = TcpListener::bind("127.0.0.1:1337").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
