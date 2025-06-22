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
    let mut stmt = tx
        .prepare("INSERT INTO abilities (name, slug, url) values (?1, ?2, ?3)")
        .map_err(|e| format!("Failed to prepare the statement: {e:?}"))?;
    let id = stmt
        .insert(rusqlite::params![
            &ability.name,
            &ability.slug,
            &ability.wiki_url
        ])
        .map_err(|e| format!("Failed to insert ability {ability:?}: {e:?}"))?;
    tracing::event!(
        tracing::Level::TRACE,
        "Inserted abbreviated ability with id {id}"
    );
    drop(stmt);
    let mut stmt = tx
        .prepare("INSERT INTO abilities_tags (ability_id, tag_name) VALUES (?1, ?2)")
        .map_err(|e| format!("Failed to prepare the statement: {e:?}"))?;
    for tag in &ability.tags {
        stmt.insert(rusqlite::params![id, tag])
            .map_err(|e| format!("Failed to insert ability tag: {e:?}"))?;
    }
    drop(stmt);
    tx.commit()?;
    Ok(())
}

fn find_abbreviated_ability_by_id(
    id: i64,
    conn: &Connection,
) -> Result<Option<AbbreviatedAbility>, String> {
    let mut stmt = conn
        .prepare("SELECT tag_name FROM abilities_tags WHERE id = ?1")
        .map_err(|_| "Failed to prepare statement".to_string())?;
    let mut rows = stmt
        .query([id])
        .map_err(|_| "Failed to execute query".to_string())?;
    let mut tags = Vec::default();
    while let Some(row) = rows
        .next()
        .map_err(|_| "Failed to fetch abilities_tags row".to_string())?
    {
        tags.push(
            row.get(0)
                .map_err(|_| "Failed to get the tag name".to_string())?,
        );
    }

    let mut stmt = conn
        .prepare("SELECT id, name, tags, wiki_url FROM abbreviated_abilities WHERE id = ?1")
        .map_err(|_| "Failed to prepare statement".to_string())?;
    let mut rows = stmt
        .query([id])
        .map_err(|_| "Failed to execute query".to_string())?;
    if let Some(row) = rows.next().map_err(|_| "Failed to fetch row".to_string())? {
        let ability = PersistedAbbreviatedAbility {
            id: row.get(0).map_err(|_| "Failed to get id".to_string())?,
            name: row.get(1).map_err(|_| "Failed to get name".to_string())?,
            slug: row.get(2).map_err(|_| "Failed to get slug".to_string())?,
            tags,
            wiki_url: row
                .get(3)
                .map_err(|_| "Failed to get wiki_url".to_string())?,
        };
        Ok(Some(ability.into()))
    } else {
        Ok(None)
    }
}

fn find_ability_tags_by_id(id: i64, conn: &Connection) -> Result<Vec<String>, String> {
    let mut stmt = conn
        .prepare("SELECT tag_name FROM abilities_tags WHERE id = ?1")
        .map_err(|_| "Failed to prepare statement".to_string())?;
    let mut rows = stmt
        .query([id])
        .map_err(|_| "Failed to execute the statement".to_string())?;
    let mut tags = Vec::default();
    while let Some(row) = rows
        .next()
        .map_err(|_| "Failed to fetch abilities_tags row".to_string())?
    {
        tags.push(
            row.get(0)
                .map_err(|_| "Failed to get the tag name".to_string())?,
        );
    }
    Ok(tags)
}

pub(crate) fn find_abbreviated_abilities_by_ids(
    ids: &[i64],
    conn: &Connection,
) -> Result<Vec<PersistedAbbreviatedAbility>, String> {
    if ids.is_empty() {
        return Ok(Vec::default());
    }

    fn repeat_question_mark(times: usize) -> String {
        let mut s = "?,".repeat(times);
        s.pop();
        s
    }
    let question_marks = repeat_question_mark(ids.len());
    let mut stmt = conn
        .prepare(&format!(
            "SELECT id, name, tags, wiki_url FROM abbreviated_abilities WHERE id IN ({})",
            question_marks
        ))
        .map_err(|_| "Failed to prepare statement".to_string())?;
    let mut rows = stmt
        .query(rusqlite::params_from_iter(ids.iter()))
        .map_err(|_| "Failed to execute query".to_string())?;
    let mut abilities = Vec::with_capacity(ids.len());
    while let Some(row) = rows
        .next()
        .map_err(|_| "Failed to feth a row".to_string())?
    {
        let id: i64 = row.get(0).map_err(|_| "Failed to get id".to_string())?;
        let tags = find_ability_tags_by_id(id, conn)
            .map_err(|_| "Failed to get tags for the ability".to_string())?;
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
) -> Result<(), String> {
    let mut conn =
        crate::db::get_connection().map_err(|e| format!("Failed to get DB connection: {e:?}"))?;
    let tx = conn
        .transaction()
        .map_err(|e| format!("Failed to start transaction: {e:?}"))?;

    let mut stmt = tx
        .prepare("UPDATE abilities SET name = ?1, url = ?2, slug = ?3 WHERE slug = ?3")
        .map_err(|e| format!("Failed to prepare the update statement: {e:?}"))?;
    let _ = stmt
        .execute(rusqlite::params![
            &ability.name,
            &ability.wiki_url,
            slug::slugify(&ability.name),
            slug
        ])
        .map_err(|e| format!("Failed to update the ability: {e:?}"))?;
    // Ensure previous tags are removed
    let mut stmt = tx
        .prepare("DELETE FROM abilities_tags WHERE id = (SELECT id FROM abilities WHERE slug = ?1)")
        .map_err(|e| format!("Failed to prepare the delete tags statement: {e:?}"))?;
    let _ = stmt
        .execute([slug])
        .map_err(|e| format!("Failed to delete tags: {e:?}"))?;
    // Add new tags
    let mut stmt = tx.prepare("INESRT INTO abilities_tags (id, tag_name) VALUES ((SELECT id FROM abilities WHERE slug = ?1), ?2)")
        .map_err(|e| format!("Failed to prepare the insert tags statement: {e:?}"))?;
    for tag in ability.tags {
        stmt.execute(rusqlite::params![slug, tag])
            .map_err(|e| format!("Failed to insert tag: {e:?}"))?;
    }

    Ok(())
}
