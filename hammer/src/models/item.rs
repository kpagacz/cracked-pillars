use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Item {
    pub(crate) name: String,
    pub(crate) slug: String,
    pub(crate) wiki_url: String,
    pub(crate) tags: Vec<String>,
    pub(crate) effects_description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct PersistedItem {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) slug: String,
    pub(crate) wiki_url: String,
    pub(crate) tags: Vec<String>,
    pub(crate) effects_description: String,
}

impl From<PersistedItem> for Item {
    fn from(value: PersistedItem) -> Self {
        Self {
            name: value.name,
            slug: value.slug,
            wiki_url: value.wiki_url,
            tags: value.tags,
            effects_description: value.effects_description,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct JsonItem {
    pub(crate) name: String,
    pub(crate) wiki_url: String,
    pub(crate) tags: Vec<String>,
    pub(crate) effects_description: String,
}

impl From<JsonItem> for Item {
    fn from(value: JsonItem) -> Self {
        let slug = slug::slugify(&value.name);
        Self {
            name: value.name,
            slug,
            wiki_url: value.wiki_url,
            tags: value.tags,
            effects_description: value.effects_description,
        }
    }
}
