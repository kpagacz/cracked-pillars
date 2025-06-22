use super::Index;
use crate::db::item;
use crate::error::Error;
use rusqlite::Connection;
use std::collections::HashMap;
use std::sync::Arc;

/// Indexes items in the database based on their tags.
pub(crate) fn index_items(conn: &Connection) -> Result<Arc<Index>, Error> {
    let items = item::find_all(conn)?;
    let mut index: HashMap<String, Vec<i64>> = HashMap::new();

    for item in items {
        for tag in item.tags {
            index
                .entry(tag)
                .and_modify(|arr| arr.push(item.id))
                .or_insert(vec![item.id]);
        }
    }

    index.values_mut().for_each(|ids| {
        ids.sort_unstable();
        ids.dedup();
    });

    Ok(Arc::new(index))
}
