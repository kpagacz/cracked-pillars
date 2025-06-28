use crate::db;
use crate::error::Error;

use super::read_abilities::read_abilities;
use super::read_items::read_items;

pub(crate) fn import_to_db() -> Result<(), Error> {
    // Load items and abilities from JSON
    let items = read_items().inspect_err(|err| tracing::warn!("Failed to read items. {err:?}"))?;
    let abilities =
        read_abilities().inspect_err(|err| tracing::warn!("Failed to read abilities. {err:?}"))?;

    let mut conn = db::get_connection()?;

    // Insert items and abilities
    for item in items {
        db::item::insert(&item, &mut conn)?;
    }
    for ability in abilities {
        db::ability::insert_abbreviated_ability(&ability)?;
    }

    Ok(())
}
