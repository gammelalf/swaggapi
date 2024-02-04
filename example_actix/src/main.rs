use std::error::Error;
use std::sync::Arc;

use actix_web::web::{Form, Json};
use actix_web::{App, HttpServer};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use swaggapi::re_exports::openapiv3::OpenAPI;
use swaggapi::{ApiContext, PageOfEverything, SwaggapiPage};

#[swaggapi::get("/index")]
pub async fn index() -> &'static str {
    "Hello World"
}

#[derive(Deserialize, JsonSchema)]
pub struct SubmitForm {}

#[swaggapi::get("/submit")]
pub async fn submit(_form: Form<SubmitForm>) -> Vec<u8> {
    Vec::new()
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
#[swaggapi::post("/json")]
pub async fn json(json: Json<JsonBody>) -> Json<JsonResponse> {
    json
}

#[actix_web::get("/openapi")]
pub async fn openapi() -> Json<Arc<OpenAPI>> {
    Json(PageOfEverything::builder().build())
}

#[derive(SwaggapiPage)]
pub struct ApiV1;

#[derive(SwaggapiPage)]
pub struct ApiV2;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(
                ApiContext::new("/api/v1")
                    .page(ApiV1)
                    .handler(submit)
                    .handler(index),
            )
            .service(
                ApiContext::new("/api/v2")
                    .page(ApiV2)
                    .handler(json)
                    .handler(index),
            )
            .service(openapi)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
