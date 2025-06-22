use super::Index;
use crate::db::ability;
use crate::error::Error;
use rusqlite::Connection;
use std::collections::HashMap;
use std::sync::Arc;

/// Index abilities in the database based on their tags.
pub(crate) fn index_abilities(conn: &Connection) -> Result<Arc<Index>, Error> {
    let abilities = ability::find_all(conn)?;
    let mut index: HashMap<String, Vec<i64>> = HashMap::new();
    for ability in abilities {
        for tag in ability.tags {
            index
                .entry(tag)
                .and_modify(|ids| ids.push(ability.id))
                .or_insert(vec![ability.id]);
        }
    }
    index.values_mut().for_each(|list| {
        list.sort();
        list.dedup();
    });
    Ok(Arc::new(index))
}
