mod rest;

use std::error::Error;

use axum::Json;
use axum::Router;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use swaggapi::utils::SchemalessJson;
use swaggapi::ApiContext;
use swaggapi::SwaggapiPage;
use swaggapi::SwaggapiPageBuilder;
use swaggapi::SwaggerUi;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let app = Router::new()
        .merge(
            ApiContext::new()
                .nest(
                    "/api/v1",
                    ApiContext::new()
                        .page(ApiV1)
                        .handler(index)
                        .handler(schemaless_json)
                        .nest(
                            "/rest",
                            ApiContext::new()
                                .tag("rest")
                                .handler(rest::create_resource)
                                .handler(rest::get_resource)
                                .handler(rest::update_resource)
                                .handler(rest::delete_resource),
                        ),
                )
                .nest(
                    "/api/v2",
                    ApiContext::new().page(&API_V2).handler(json).handler(index),
                ),
        )
        .merge(
            SwaggerUi::default()
                .page("API v1", ApiV1)
                .page("API v2", &API_V2),
        );

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
