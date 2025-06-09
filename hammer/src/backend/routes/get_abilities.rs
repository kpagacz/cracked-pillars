use std::sync::Arc;

use axum::Json;
use serde_json::Value;
use serde_json::json;

use crate::backend::ability::Ability;

pub(super) async fn get_abilities(abilities: Arc<Vec<Ability>>) -> Json<Value> {
    Json(json!(abilities.to_vec()))
}
