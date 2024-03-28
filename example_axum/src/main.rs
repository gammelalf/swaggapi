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
    description = "This is the first revision of my application's api"
)]
pub struct ApiV1;

// Builder style api page
pub static API_V2: SwaggapiPageBuilder = SwaggapiPageBuilder::new()
    .title("My application")
    .description("This is the second revision of my application's api");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .merge(
            ApiContext::new("/api/v1")
                .tag("v1")
                .page(ApiV1)
                .handler(index)
                .handler(schemaless_json),
        )
        .merge(
            ApiContext::new("/api/v2")
                .page(&API_V2)
                .handler(json)
                .handler(index),
        )
        .merge(
            SwaggerUi::default()
                .page("API v1", "openapi_v1.json", ApiV1)
                .page("API v2", "openapi_v2.json", &API_V2),
        );

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
