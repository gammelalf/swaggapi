use std::error::Error;
use std::sync::Arc;

use actix_web::web::{Data, Form, Json, Path};
use actix_web::{App, HttpResponse, HttpServer};
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

    let config = Data::new(utoipa_swagger_ui::Config::<'static>::new(["/openapi"]));

    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
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
            .service(get_swagger_ui)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

#[actix_web::get("/swagger-ui/{_:.*}")]
async fn get_swagger_ui(path: Path<String>) -> HttpResponse {
    match swaggapi::swagger::file(&path.into_inner()) {
        Some(file) => HttpResponse::Ok().body(file.to_vec()),
        None => HttpResponse::NotFound().finish(),
    }
}
