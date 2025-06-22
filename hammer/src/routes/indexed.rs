use crate::db;
use crate::error::Error;
use crate::models::IndexedEntity;
use axum::extract::{Json, State};
use axum_extra::extract::Query;
use rusqlite::Connection;

use crate::db::ability;
use crate::db::item;
use crate::indexing::Index;
use crate::models::FilterParams;
use std::collections::HashSet;
use std::sync::Arc;

#[axum::debug_handler]
pub(super) async fn get(
    Query(params): Query<FilterParams>,
    State(indexed_abilities): State<Arc<Index>>,
    State(indexed_items): State<Arc<Index>>,
) -> Result<Json<Vec<IndexedEntity>>, Error> {
    let conn = db::get_connection()?;
    Ok(Json(get_indexed(
        params,
        &indexed_abilities,
        &indexed_items,
        &conn,
    )?))
}

fn get_indexed(
    params: FilterParams,
    abilities: &Index,
    items: &Index,
    conn: &Connection,
) -> Result<Vec<IndexedEntity>, Error> {
    let tags = params.tags;
    let filter_logic = params.filter_logic;
    match filter_logic.as_str() {
        "or" => get_with_or_filter(&tags, abilities, items, conn),
        "and" => get_with_and_filter(&tags, abilities, items, conn),
        _ => {
            tracing::warn!("Unknown filter logic: {}", filter_logic);
            Ok(Vec::new())
        }
    }
}

fn get_with_or_filter(
    tags: &[String],
    abilities: &Index,
    items: &Index,
    conn: &Connection,
) -> Result<Vec<IndexedEntity>, Error> {
    let mut indexed_entities = Vec::new();
    for tag in tags {
        if let Some(ids) = abilities.get(tag) {
            indexed_entities.extend(
                ability::find_abbreviated_abilities_by_ids(ids, conn)?
                    .into_iter()
                    .map(IndexedEntity::from),
            );
        }
        if let Some(ids) = items.get(tag) {
            indexed_entities.extend(
                item::find_by_ids(ids, conn)?
                    .into_iter()
                    .map(IndexedEntity::from),
            );
        }
    }
    indexed_entities.sort_by(|a, b| a.name.cmp(&b.name));
    indexed_entities.dedup_by(|a, b| a.slug == b.slug);
    Ok(indexed_entities)
}

fn get_with_and_filter(
    tags: &[String],
    abilities: &Index,
    items: &Index,
    conn: &Connection,
) -> Result<Vec<IndexedEntity>, Error> {
    let mut indexed_entities = Vec::new();
    let mut and_abilities = ability::find_all(conn)?
        .into_iter()
        .map(IndexedEntity::from)
        .collect::<HashSet<_>>();
    let mut and_items = item::find_all(conn)?
        .into_iter()
        .map(IndexedEntity::from)
        .collect::<HashSet<_>>();
    for tag in tags {
        if let Some(ids) = abilities.get(tag) {
            let abs = ability::find_abbreviated_abilities_by_ids(ids, conn)?
                .into_iter()
                .map(IndexedEntity::from)
                .collect::<HashSet<_>>();
            and_abilities = and_abilities
                .intersection(&abs)
                .cloned()
                .collect::<HashSet<_>>();
            tracing::trace!("Finished updating the and abilities for tag: {tag:?}");
        }
        if let Some(ids) = items.get(tag) {
            let itms = item::find_by_ids(ids, conn)?
                .into_iter()
                .map(IndexedEntity::from)
                .collect::<HashSet<_>>();
            and_items = and_items
                .intersection(&itms)
                .cloned()
                .collect::<HashSet<_>>();
            tracing::trace!("Finished updating the and items for tag: {tag:?}");
        }
    }
    indexed_entities.extend(and_abilities);
    indexed_entities.extend(and_items);
    indexed_entities.sort_by(|a, b| a.name.cmp(&b.name));
    indexed_entities.dedup_by(|a, b| a.slug == b.slug);
    Ok(indexed_entities)
}
