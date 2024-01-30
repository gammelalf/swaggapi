use std::error::Error;
use std::sync::Arc;

use axum::{Form, Json, Router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use swaggapi::re_exports::openapiv3::OpenAPI;
use swaggapi::{ApiContext, PageOfEverything, SwaggapiPage};
use tokio::net::TcpListener;

#[swaggapi::get("/index")]
pub async fn index() -> &'static str {
    "Hello world"
}

#[derive(Deserialize, JsonSchema)]
pub struct SubmitForm {}

/*
#[swaggapi::get("/submit")]
pub async fn submit(_form: Form<SubmitForm>) -> Vec<u8> {
    Vec::new()
}
*/

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
#[swaggapi::post("/json")]
pub async fn json(_json: Json<JsonBody>) -> Json<JsonResponse> {
    todo!()
}

pub async fn openapi() -> Json<Arc<OpenAPI>> {
    Json(PageOfEverything::builder().build())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .merge(ApiContext::new("/api").handler(json).handler(index))
        .route("/openapi", axum::routing::get(openapi));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
