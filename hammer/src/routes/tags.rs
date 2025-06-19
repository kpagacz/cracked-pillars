use axum::Json;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) async fn get(index: Arc<HashMap<String, Vec<usize>>>) -> Json<Value> {
    Json(json!(index.keys().cloned().collect::<Vec<String>>()))
}
