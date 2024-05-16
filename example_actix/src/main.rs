mod rest;

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
use swaggapi::SwaggapiPageBuilder;
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

// Derive style api page
#[derive(SwaggapiPage)]
#[page(
    title = "My application",
    description = "This is the first revision of my application's api",
    filename = "openapi_v1.json"
)]
pub struct ApiV1;

// Builder style api page
pub static API_V2: SwaggapiPageBuilder = SwaggapiPageBuilder::new()
    .title("My application")
    .description("This is the second revision of my application's api")
    .filename("openapi_v2.json");

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
                    .handler(schemaless_json)
                    .service(
                        ApiContext::new("/rest")
                            .tag("rest")
                            .handler(rest::create_resource)
                            .handler(rest::get_resource)
                            .handler(rest::update_resource)
                            .handler(rest::delete_resource),
                    ),
            )
            .service(
                ApiContext::new("/api/v2")
                    .page(&API_V2)
                    .handler(json)
                    .handler(index),
            )
            .service(
                SwaggerUi::default()
                    .page("API v1", ApiV1)
                    .page("API v2", &API_V2),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
