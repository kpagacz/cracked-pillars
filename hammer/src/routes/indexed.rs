use crate::db;
use crate::error::Error;
use crate::models::IndexedEntity;
use axum::extract::Json;
use axum_extra::extract::Query;
use rusqlite::Connection;

use crate::models::FilterParams;

#[axum::debug_handler]
pub(super) async fn get(
    Query(params): Query<FilterParams>,
) -> Result<Json<Vec<IndexedEntity>>, Error> {
    let conn = db::get_connection()?;
    Ok(Json(get_indexed(params, &conn)?))
}

fn get_indexed(params: FilterParams, conn: &Connection) -> Result<Vec<IndexedEntity>, Error> {
    let tags = params.tags;
    let filter_logic = params.filter_logic;
    match filter_logic.as_str() {
        "or" => get_with_or_filter(&tags, conn),
        "and" => get_with_and_filter(&tags, conn),
        _ => {
            tracing::warn!("Unsupported filter logic: {filter_logic}");
            Err(Error(
                format!("Unsupported filter logic {filter_logic}"),
                crate::error::ErrorType::Runtime,
            ))
        }
    }
}

fn get_with_or_filter(tags: &[String], conn: &Connection) -> Result<Vec<IndexedEntity>, Error> {
    let placeholder = (0..tags.len()).map(|_| "?").collect::<Vec<_>>().join(",");
    let mut stmt = conn.prepare_cached(&format!(
        "SELECT ability_id FROM abilities_tags WHERE tag_name IN ({placeholder})"
    ))?;
    let mut rows = stmt.query(rusqlite::params_from_iter(tags))?;
    let mut abilities_ids = vec![];
    while let Some(row) = rows.next()? {
        let id: i64 = row.get(0)?;
        abilities_ids.push(id);
    }
    let abilities = db::ability::find_abbreviated_abilities_by_ids(&abilities_ids, conn)?
        .into_iter()
        .map(IndexedEntity::from);

    let mut stmt = conn.prepare_cached(&format!(
        "SELECT item_id FROM items_tags WHERE tag_name IN ({placeholder})"
    ))?;
    let mut rows = stmt.query(rusqlite::params_from_iter(tags))?;
    let mut items_ids = vec![];
    while let Some(row) = rows.next()? {
        let id = row.get(0)?;
        items_ids.push(id);
    }
    let items = db::item::find_by_ids(&items_ids, conn)?
        .into_iter()
        .map(IndexedEntity::from);
    Ok(abilities.chain(items).collect())
}

fn get_with_and_filter(tags: &[String], conn: &Connection) -> Result<Vec<IndexedEntity>, Error> {
    let placeholder = (0..tags.len()).map(|_| "?").collect::<Vec<_>>().join(",");

    // Abilities
    let ability_stmt = format!(
        "SELECT ability_id FROM abilities_tags WHERE tag_name IN ({placeholder}) GROUP BY ability_id HAVING COUNT(DISTINCT tag_name) = {}",
        tags.len()
    );
    let mut stmt = conn
        .prepare_cached(&ability_stmt)
        .inspect_err(|err| tracing::warn!("Failed to prepare the stmt: {err:?}"))?;
    let mut rows = stmt
        .query(rusqlite::params_from_iter(tags))
        .inspect_err(|err| tracing::warn!("Failed to query the db: {err:?}"))?;
    let mut abilities_id = vec![];
    while let Some(row) = rows.next()? {
        abilities_id.push(row.get(0)?);
    }
    let abilities = db::ability::find_abbreviated_abilities_by_ids(&abilities_id, conn)?;

    // Items
    let items_stmt = format!(
        "SELECT item_id FROM items_tags WHERE tag_name IN ({placeholder}) GROUP BY item_id HAVING COUNT(DISTINCT tag_name) = {}",
        tags.len()
    );
    let mut stmt = conn
        .prepare_cached(&items_stmt)
        .inspect_err(|err| tracing::warn!("Failed to prepare the statement: {err:?}"))?;
    let mut rows = stmt
        .query(rusqlite::params_from_iter(tags))
        .inspect_err(|err| tracing::warn!("Failed to query the db: {err:?}"))?;
    let mut items_ids = vec![];
    while let Some(row) = rows.next()? {
        items_ids.push(row.get(0)?);
    }
    let items = db::item::find_by_ids(&items_ids, conn)?;

    Ok(abilities
        .into_iter()
        .map(IndexedEntity::from)
        .chain(items.into_iter().map(IndexedEntity::from))
        .collect())
}
