use crate::db;
use crate::error::Error;
use crate::models::AbbreviatedAbility;
use crate::models::CONFIG;
use crate::models::Item;

use super::read_abilities::read_abilities;
use super::read_items::read_items;

fn get_tags(abilities: &[AbbreviatedAbility], items: &[Item]) -> Vec<String> {
    let mut tags = Vec::new();
    for ability in abilities {
        tags.extend(ability.tags.clone());
    }
    for item in items {
        tags.extend(item.tags.clone());
    }
    tags.sort_unstable();
    tags.dedup();
    tags
}

pub(crate) fn import_to_db() -> Result<(), Error> {
    // Remove old db
    std::fs::remove_file(&CONFIG.db_path)?;
    // Load items and abilities from JSON
    let items = read_items()?;
    let abilities = read_abilities()?;
    let tags = get_tags(&abilities, &items);

    // Insert tags
    let mut conn = db::get_connection()?;
    db::tag::insert_batch(&tags, &conn)?;

    // Insert items and abilities
    for item in items {
        db::item::insert(&item, &mut conn)?;
    }
    for ability in abilities {
        db::ability::insert_abbreviated_ability(&ability)?;
    }

    Ok(())
}
