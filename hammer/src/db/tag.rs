use rusqlite::Connection;

use crate::error::Error;

pub(crate) fn insert_batch(tags: &[String], conn: &Connection) -> Result<(), Error> {
    let placeholders = tags.iter().map(|_| "(?)").collect::<Vec<_>>().join(", ");
    let mut stmt = conn.prepare(&format!("INSERT INTO tags (name) VALUES {placeholders}"))?;
    stmt.execute(rusqlite::params_from_iter(tags.iter()))?;

    Ok(())
}

pub(crate) fn find_all(conn: &Connection) -> Result<Vec<String>, Error> {
    let mut stmt = conn.prepare_cached("SELECT name FROM tags")?;
    let mut rows = stmt.query([])?;
    let mut tags = Vec::new();
    while let Some(row) = rows.next()? {
        tags.push(row.get(0)?);
    }
    Ok(tags)
}
