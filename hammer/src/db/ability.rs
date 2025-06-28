use crate::{
    error::Error,
    models::{AbbreviatedAbility, PersistedAbbreviatedAbility},
};
use rusqlite::Connection;

pub(crate) fn find_all(conn: &Connection) -> Result<Vec<PersistedAbbreviatedAbility>, Error> {
    let mut stmt = conn.prepare("SELECT * FROM abilities")?;
    let mut rows = stmt.query([])?;
    let mut abilities = Vec::new();
    while let Some(row) = rows.next()? {
        abilities.push(from_row(row, conn)?);
    }
    Ok(abilities)
}

fn from_row(row: &rusqlite::Row, conn: &Connection) -> Result<PersistedAbbreviatedAbility, Error> {
    let id = row.get(0)?;
    let slug = row.get(1)?;
    let name = row.get(2)?;
    let wiki_url = row.get(3)?;

    let mut stmt = conn.prepare("SELECT tag_name FROM abilities_tags WHERE ability_id=?1")?;
    let mut tags = Vec::new();
    let mut rows = stmt.query([id])?;
    while let Some(row) = rows.next()? {
        tags.push(row.get(0)?);
    }

    Ok(PersistedAbbreviatedAbility {
        id,
        slug,
        name,
        tags,
        wiki_url,
    })
}

pub(crate) fn insert_abbreviated_ability(ability: &AbbreviatedAbility) -> Result<(), Error> {
    let mut conn =
        crate::db::get_connection().map_err(|e| format!("Failed to get DB connection: {e:?}"))?;
    let tx = conn.transaction().map_err(|e| {
        format!("Failed to start transaction for inserting abbreviated ability: {e:?}")
    })?;
    let mut stmt = tx.prepare("INSERT INTO abilities (name, slug, url) values (?1, ?2, ?3)")?;
    let id = stmt
        .insert(rusqlite::params![
            &ability.name,
            &ability.slug,
            &ability.wiki_url
        ])
        .inspect_err(|err| tracing::warn!("Failed to insert ability into table. {err:?}"))?;
    drop(stmt);
    let mut stmt =
        tx.prepare("INSERT INTO abilities_tags (ability_id, tag_name) VALUES (?1, ?2)")?;
    for tag in &ability.tags {
        stmt.insert(rusqlite::params![id, tag])
            .inspect_err(|err| tracing::warn!("Failed to insert abilities tag {tag}. {err:?}"))?;
    }
    drop(stmt);
    tx.commit()?;
    Ok(())
}

fn find_ability_tags_by_id(id: i64, conn: &Connection) -> Result<Vec<String>, Error> {
    let mut stmt = conn.prepare("SELECT tag_name FROM abilities_tags WHERE ability_id=?1")?;
    let mut rows = stmt.query([id])?;
    let mut tags = Vec::default();
    while let Some(row) = rows.next()? {
        tags.push(row.get(0)?);
    }
    Ok(tags)
}

pub(crate) fn find_abbreviated_abilities_by_ids(
    ids: &[i64],
    conn: &Connection,
) -> Result<Vec<PersistedAbbreviatedAbility>, Error> {
    if ids.is_empty() {
        return Ok(Vec::default());
    }

    let placeholder = ids
        .iter()
        .map(|id| format!("{id}"))
        .collect::<Vec<_>>()
        .join(",");
    let mut stmt = conn.prepare(&format!(
        "SELECT id, name, slug, url FROM abilities WHERE id IN ({placeholder})"
    ))?;
    let mut rows = stmt.query([])?;
    let mut abilities = Vec::with_capacity(ids.len());
    while let Some(row) = rows.next()? {
        let id: i64 = row.get(0)?;
        let tags = find_ability_tags_by_id(id, conn)?;
        abilities.push(PersistedAbbreviatedAbility {
            id,
            name: row.get(1).map_err(|_| "Failed to get name".to_string())?,
            slug: row.get(2).map_err(|_| "Failed to get slug".to_string())?,
            tags,
            wiki_url: row
                .get(3)
                .map_err(|_| "Failed to get wiki_url".to_string())?,
        });
    }
    tracing::trace!("Done with returning abilities by id");
    Ok(abilities)
}

pub(crate) fn delete_abbreviated_ability_by_slug(slug: &str) -> Result<(), String> {
    let conn =
        crate::db::get_connection().map_err(|e| format!("Failed to get DB connection: {e:?}"))?;
    let mut stmt = conn
        .prepare("DELETE from abilities where slug = ?1")
        .map_err(|e| format!("Failed to prepare the delete statement: {e:?}"))?;
    stmt.execute([slug])
        .map_err(|e| format!("Failed to delete the ability: {e:?}"))?;
    Ok(())
}

pub(crate) fn update_abbreviated_ability_by_slug(
    slug: &str,
    ability: AbbreviatedAbility,
) -> Result<(), Error> {
    let mut conn = crate::db::get_connection()?;
    let tx = conn.transaction()?;

    let mut stmt = tx
        .prepare("UPDATE abilities SET name = ?1, url = ?2, slug = ?3 WHERE slug = ?4")
        .map_err(|e| format!("Failed to prepare the update statement: {e:?}"))?;
    let _ = stmt
        .execute(rusqlite::params![
            &ability.name,
            &ability.wiki_url,
            slug::slugify(&ability.name),
            slug
        ])
        .map_err(|e| format!("Failed to update the ability: {e:?}"))?;
    drop(stmt);
    // Ensure previous tags are removed
    let mut stmt = tx
        .prepare("DELETE FROM abilities_tags WHERE ability_id=(SELECT id FROM abilities WHERE slug = ?1)")
        .map_err(|e| format!("Failed to prepare the delete tags statement: {e:?}"))?;
    let _ = stmt
        .execute([slug])
        .map_err(|e| format!("Failed to delete tags: {e:?}"))?;
    drop(stmt);
    // Add new tags
    let mut stmt = tx.prepare("INSERT INTO abilities_tags (ability_id, tag_name) VALUES ((SELECT id FROM abilities WHERE slug=?1), ?2)")
        .map_err(|e| format!("Failed to prepare the insert tags statement: {e:?}"))?;
    for tag in ability.tags {
        stmt.execute(rusqlite::params![slug, tag])
            .map_err(|e| format!("Failed to insert tag: {e:?}"))?;
    }

    drop(stmt);
    tx.commit()?;
    Ok(())
}

pub(crate) fn update_tags_by_slug(
    slug: &str,
    new_tags: Vec<String>,
    conn: &mut Connection,
) -> Result<Vec<String>, Error> {
    let tx = conn.transaction()?;

    let mut stmt = tx.prepare_cached(
        "DELETE FROM abilities_tags WHERE ability_id=(SELECT id FROM abilities WHERE slug=?1)",
    )?;
    stmt.execute([slug])?;
    drop(stmt);

    let mut stmt = tx.prepare_cached("INSERT INTO abilities_tags (ability_id, tag_name) VALUES ((SELECT id FROM abilities WHERE slug=?1), ?2)")?;
    for tag in &new_tags {
        stmt.execute([slug, tag])?;
    }
    drop(stmt);
    tx.commit()?;

    Ok(new_tags)
}

pub(crate) fn find_by_slug(
    slug: &str,
    conn: &Connection,
) -> Result<Option<PersistedAbbreviatedAbility>, Error> {
    let mut stmt = conn.prepare_cached("SELECT * FROM abilities WHERE slug=?1")?;
    let mut row = stmt.query([slug])?;
    Ok(row.next()?.and_then(|row| from_row(row, conn).ok()))
}
