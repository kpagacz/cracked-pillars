use std::sync::Arc;

use crate::backend::{
    ability::Ability,
    db::{get_connection, synchronize_db},
    index_abilities::index_abilities,
    read_abilities::read_abilities,
};
use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

async fn get_abilities(abilities: Arc<Vec<Ability>>) -> Json<Value> {
    Json(json!(abilities.to_vec()))
}

pub(crate) fn get_backend_routes() -> Router {
    let db_connection = get_connection().unwrap();
    synchronize_db(&db_connection).unwrap();

    let abilities = Arc::new(read_abilities().unwrap());
    let index = Arc::new(index_abilities(&abilities));

    Router::new().route(
        "/get-abilities",
        get({
            let abilities = Arc::clone(&abilities);
            move || get_abilities(abilities)
        }),
    )
}
