use std::error::Error;

use actix_web::web::{Form, Json};
use actix_web::{App, HttpServer};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use swaggapi::SwaggerUi;
use swaggapi::{ApiContext, SwaggapiPage};

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

#[derive(SwaggapiPage)]
pub struct ApiV1;

#[derive(SwaggapiPage)]
pub struct ApiV2;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    HttpServer::new(move || {
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
            .service(
                SwaggerUi::default()
                    .page("API v1", "openapi_v1.json", ApiV1)
                    .page("API v2", "openapi_v2.json", ApiV2),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
