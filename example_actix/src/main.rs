use std::collections::HashMap;
use std::error::Error;

use actix_web::web::{Form, Json};
use actix_web::{App, HttpServer};
use schemars::gen::{SchemaGenerator, SchemaSettings};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use swaggapi::convert_schema;
use swaggapi::handler::Handler;

#[derive(Deserialize, JsonSchema)]
pub struct SubmitForm {}

#[swaggapi::get("", "/submit")]
pub async fn submit(_form: Form<SubmitForm>) -> &'static str {
    "YaY"
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

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut set = SchemaSettings::openapi3();
    set.visitors = Vec::new();
    let mut gen = SchemaGenerator::new(set);

    serde_json::to_writer_pretty(std::io::stdout(), &submit.description(&mut gen).build()).unwrap();
    serde_json::to_writer_pretty(std::io::stdout(), &json.description(&mut gen).build()).unwrap();

    let components: HashMap<_, _> = gen
        .take_definitions()
        .into_iter()
        .map(|(k, v)| (k, convert_schema(v)))
        .collect();
    serde_json::to_writer_pretty(std::io::stdout(), &components).unwrap();

    HttpServer::new(|| App::new().service(submit.as_dyn()).service(json.as_dyn()))
        .bind("127.0.0.1:1337")?
        .run()
        .await?;

    Ok(())
}
