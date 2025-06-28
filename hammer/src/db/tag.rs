use rusqlite::Connection;

use crate::error::Error;
use crate::models::Tag;
use rusqlite::Row;

fn from_row(row: &Row) -> Result<Tag, Error> {
    Ok(Tag {
        name: row.get(0)?,
        description: row.get(1)?,
    })
}

pub(crate) fn find_all(conn: &Connection) -> Result<Vec<Tag>, Error> {
    let mut stmt = conn.prepare_cached("SELECT name,description FROM tags")?;
    let mut rows = stmt.query([])?;
    let mut tags = Vec::new();
    while let Some(row) = rows.next()? {
        tags.push(from_row(row)?);
    }
    Ok(tags)
}
