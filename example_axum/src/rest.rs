//! Example of a very basic REST api
//!
//! This uses `Json` both as extractor as well as response.
//! It also tests very basic path parameters.

use std::collections::HashMap;
use std::sync::Mutex;

use axum::extract::Path;
use axum::Json;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use swaggapi::delete;
use swaggapi::get;
use swaggapi::post;
use swaggapi::put;
use uuid::Uuid;

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct Resource {}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PathUuid {
    uuid: Uuid,
}

#[post("/resource")]
pub async fn create_resource(json: Json<Resource>) -> Json<Uuid> {
    let uuid = Uuid::new_v4();
    modify(move |map| map.insert(uuid, json.0));
    Json(uuid)
}

#[get("/resource/:uuid")]
pub async fn get_resource(path: Path<Uuid>) -> Json<Option<Resource>> {
    Json(modify(move |map| map.get(&path).cloned()))
}

#[put("/resource/:uuid")]
pub async fn update_resource(path: Path<PathUuid>, json: Json<Resource>) -> Json<bool> {
    Json(modify(move |map| {
        map.contains_key(&path.uuid) && {
            map.insert(path.uuid, json.0);
            true
        }
    }))
}

#[delete("/resource/:uuid")]
pub async fn delete_resource(path: Path<PathUuid>) -> Json<bool> {
    Json(modify(move |map| map.remove(&path.uuid).is_some()))
}

fn modify<R>(f: impl FnOnce(&mut HashMap<Uuid, Resource>) -> R) -> R {
    static RESOURCES: Mutex<Option<HashMap<Uuid, Resource>>> = Mutex::new(None);
    let mut guard = RESOURCES.lock().unwrap();
    let map = guard.get_or_insert(HashMap::new());
    f(map)
}
