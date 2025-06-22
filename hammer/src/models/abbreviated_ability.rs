use super::Ability;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AbbreviatedAbility {
    pub(crate) name: String,
    pub(crate) slug: String,
    pub(crate) tags: Vec<String>,
    pub(crate) wiki_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PersistedAbbreviatedAbility {
    pub(crate) id: i64,
    pub(crate) slug: String,
    pub(crate) name: String,
    pub(crate) tags: Vec<String>,
    pub(crate) wiki_url: String,
}

impl From<PersistedAbbreviatedAbility> for AbbreviatedAbility {
    fn from(value: PersistedAbbreviatedAbility) -> Self {
        Self {
            name: value.name,
            slug: value.slug,
            tags: value.tags,
            wiki_url: value.wiki_url,
        }
    }
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
        let slug = slug::slugify(&name);
        Self {
            name,
            slug,
            tags,
            wiki_url,
        }
    }
}
