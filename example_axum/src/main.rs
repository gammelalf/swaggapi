use std::borrow::Cow;
use std::error::Error;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::Path;
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{AppendHeaders, IntoResponse, Response};
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

#[derive(SwaggapiPage)]
pub struct ApiV1;

#[derive(SwaggapiPage)]
pub struct ApiV2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .merge(ApiContext::new("/api/v1").page(ApiV1).handler(index))
        .merge(
            ApiContext::new("/api/v2")
                .page(ApiV1)
                .handler(json)
                .handler(index),
        )
        .route("/openapi", axum::routing::get(openapi))
        .route("/swagger-ui/*_", axum::routing::get(get_swagger_ui));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_swagger_ui(Path(path): Path<String>) -> Response {
    let file = swaggapi::swagger::file(&path);
    Response::builder()
        .status(if file.is_some() {
            StatusCode::OK
        } else {
            StatusCode::NOT_FOUND
        })
        .header(
            CONTENT_TYPE,
            mime_guess::from_path(&path)
                .first_or_octet_stream()
                .to_string(),
        )
        .body(Body::from(file.unwrap_or_default()))
        .unwrap()
}
