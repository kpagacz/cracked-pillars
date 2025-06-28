use crate::error::Error;
use rusqlite::Connection;

use crate::models::{Item, PersistedItem};

pub(crate) fn insert(item: &Item, conn: &mut Connection) -> Result<(), Error> {
    let tx = conn.transaction()?;
    let mut stmt = tx.prepare(
        "INSERT INTO items (name, slug, wiki_url, effects_description) values (?1, ?2, ?3, ?4)",
    )?;
    let id = stmt
        .insert(rusqlite::params![
            &item.name,
            &item.slug,
            &item.wiki_url,
            &item.effects_description
        ])
        .inspect_err(|err| tracing::warn!("Failed to insert an item into the table. {err:?}"))?;
    drop(stmt);
    let mut stmt = tx.prepare("INSERT INTO items_tags (item_id, tag_name) VALUES (?1, ?2)")?;
    for tag_name in &item.tags {
        stmt.insert(rusqlite::params![id, tag_name])
            .inspect_err(|err| {
                tracing::warn!("Failed to insert {tag_name} to items_tags table. Err: {err:?}")
            })?;
    }
    drop(stmt);
    tx.commit()?;
    Ok(())
}

fn from_row(row: &rusqlite::Row, conn: &Connection) -> Result<PersistedItem, Error> {
    let id = row.get(0)?;
    let name = row.get(1)?;
    let slug = row.get(2)?;
    let wiki_url = row.get(3)?;
    let effects_description = row.get(4).unwrap_or("".to_string());
    let mut tags = Vec::default();
    let mut stmt = conn.prepare("SELECT item_id,tag_name FROM items_tags WHERE item_id=?1")?;
    let mut rows = stmt.query(rusqlite::params![id])?;
    while let Some(row) = rows.next()? {
        let tag_name = row.get(1)?;
        tags.push(tag_name);
    }
    Ok(PersistedItem {
        id,
        name,
        slug,
        wiki_url,
        tags,
        effects_description,
    })
}

pub(crate) fn find_by_slug(slug: &str, conn: &Connection) -> Result<Option<PersistedItem>, Error> {
    let mut stmt =
        conn.prepare("SELECT id,name,slug,wiki_url,effects_description FROM items WHERE slug=?1")?;
    let mut rows = stmt.query(rusqlite::params![slug])?;
    Ok(rows.next()?.and_then(|row| from_row(row, conn).ok()))
}

pub(crate) fn find_all(conn: &Connection) -> Result<Vec<PersistedItem>, Error> {
    let mut stmt = conn.prepare("SELECT id,name,slug,wiki_url,effects_description FROM items")?;
    let mut rows = stmt.query([])?;
    let mut items = Vec::new();
    while let Some(row) = rows.next()? {
        items.push(from_row(row, conn)?);
    }
    Ok(items)
}

pub(crate) fn delete(slug: &str, conn: &Connection) -> Result<(), Error> {
    let mut stmt = conn.prepare("DELETE FROM items WHERE slug=?1")?;
    stmt.execute(rusqlite::params![slug])?;
    Ok(())
}

pub(crate) fn update(slug: &str, item: &Item, conn: &mut Connection) -> Result<(), Error> {
    let tx = conn.transaction()?;
    let mut stmt = tx.prepare(
        "UPDATE items set name=?1, slug=?2, wiki_url=?3, effects_description=?4 WHERE slug=?5",
    )?;
    let _ = stmt.execute(rusqlite::params![
        item.name,
        item.slug,
        item.wiki_url,
        item.effects_description,
        slug
    ])?;
    drop(stmt);
    let mut stmt =
        tx.prepare("DELETE FROM items_tags WHERE item_id=(SELECT id FROM items WHERE slug=?1)")?;
    let _ = stmt.execute(rusqlite::params![slug])?;
    drop(stmt);
    let mut stmt = tx.prepare("INSERT INTO items_tags (item_id, tag_name) VALUES ((SELECT id FROM items WHERE slug=?1), ?2)")?;
    for tag_name in &item.tags {
        stmt.insert(rusqlite::params![slug, tag_name])?;
    }
    drop(stmt);
    tx.commit()?;
    Ok(())
}

pub(crate) fn find_by_ids(ids: &[i64], conn: &Connection) -> Result<Vec<PersistedItem>, Error> {
    let placeholder = ids
        .iter()
        .map(|id| format!("{id}"))
        .collect::<Vec<String>>()
        .join(",");
    let mut stmt = conn.prepare_cached(&format!(
        "SELECT id,name,slug,wiki_url FROM items WHERE id IN ({placeholder})"
    ))?;
    let mut rows = stmt.query([])?;
    let mut items = Vec::new();
    while let Some(row) = rows.next()? {
        items.push(from_row(row, conn)?);
    }
    Ok(items)
}

pub(crate) fn update_tags_by_slug(
    slug: &str,
    new_tags: Vec<String>,
    conn: &mut Connection,
) -> Result<Vec<String>, Error> {
    let tx = conn.transaction()?;
    let mut stmt = tx.prepare_cached(
        "DELETE FROM items_tags WHERE item_id=(SELECT id FROM items WHERE slug=?1)",
    )?;
    stmt.execute([slug])?;
    drop(stmt);

    let mut stmt = tx.prepare_cached("INSERT INTO items_tags (item_id, tag_name) VALUES ((SELECT id FROM items WHERE slug=?1), ?2)")?;
    for tag in &new_tags {
        stmt.execute([slug, tag])?;
    }
    drop(stmt);
    tx.commit()?;
    Ok(new_tags)
}
