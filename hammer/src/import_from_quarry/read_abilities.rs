use crate::error::Error;
use crate::models::{AbbreviatedAbility, Ability, CONFIG};
use std::path::Path;

pub(super) fn read_abilities() -> Result<Vec<AbbreviatedAbility>, Error> {
    let path = &CONFIG.abilities_path;
    let path = Path::new(path);
    tracing::trace!("abilities path: {path:?}");

    let file_contents = std::fs::read_to_string(path)
        .map_err(|err| format!("Failed to read the abilities files: {err:?}"))?;
    serde_json::from_str::<Vec<Ability>>(&file_contents)
        .map_err(|err| format!("Failed to parse abilities from string: {err:?}").into())
        .map(|abilities| {
            abilities
                .into_iter()
                .map(AbbreviatedAbility::from)
                .collect()
        })
}
