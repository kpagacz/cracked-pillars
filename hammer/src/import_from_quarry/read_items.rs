use crate::error::Error;
use crate::models::CONFIG;
use crate::models::{Item, JsonItem};
use std::path::Path;

pub(super) fn read_items() -> Result<Vec<Item>, Error> {
    let path = &CONFIG.items_path;
    let path = Path::new(path);
    tracing::trace!("items path: {path:?}");

    let file_contents = std::fs::read_to_string(path)
        .map_err(|err| format!("Failed to read the items files: {err:?}"))?;
    serde_json::from_str::<Vec<JsonItem>>(&file_contents)
        .map_err(|err| format!("Failed to parse items from string: {err:?}").into())
        .map(|items| items.into_iter().map(Item::from).collect())
}
