use std::error::Error;

use actix_web::web::Form;
use actix_web::web::Json;
use actix_web::App;
use actix_web::HttpServer;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use swaggapi::utils::SchemalessJson;
use swaggapi::ApiContext;
use swaggapi::SwaggapiPage;
use swaggapi::SwaggerUi;

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

#[swaggapi::post("/json")]
pub async fn schemaless_json(json2: SchemalessJson<()>) -> SchemalessJson<()> {
    json2
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
                    .handler(index)
                    .handler(schemaless_json),
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
