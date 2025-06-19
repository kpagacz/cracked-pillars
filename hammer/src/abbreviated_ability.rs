use crate::ability::Ability;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AbbreviatedAbility {
    pub(crate) name: String,
    pub(crate) tags: Vec<String>,
    pub(crate) wiki_url: String,
}

impl From<Ability> for AbbreviatedAbility {
    fn from(value: Ability) -> Self {
        let name = value.name.clone();
        let wiki_url = value.url.clone();
        let tags: Vec<String> = value
            .effects
            .iter()
            .flat_map(|effect| effect.tags.clone())
            .collect();
        Self {
            name,
            tags,
            wiki_url,
        }
    }
}
