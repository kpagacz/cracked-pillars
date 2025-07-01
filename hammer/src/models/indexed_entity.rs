use crate::models::{PersistedAbbreviatedAbility, PersistedItem};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Hash)]
pub(crate) struct IndexedEntity {
    pub(crate) name: String,
    pub(crate) slug: String,
    pub(crate) wiki_url: String,
    pub(crate) tags: Vec<String>,
    #[serde(rename = "type")]
    pub(crate) entity_type: IndexedEntityType,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Hash)]
pub(crate) enum IndexedEntityType {
    #[serde(rename = "item")]
    Item,
    #[serde(rename = "ability")]
    Ability,
}

impl From<PersistedItem> for IndexedEntity {
    fn from(value: PersistedItem) -> Self {
        Self {
            name: value.name,
            slug: value.slug,
            wiki_url: value.wiki_url,
            tags: value.tags,
            entity_type: IndexedEntityType::Item,
        }
    }
}

impl From<PersistedAbbreviatedAbility> for IndexedEntity {
    fn from(value: PersistedAbbreviatedAbility) -> Self {
        Self {
            name: value.name,
            slug: value.slug,
            wiki_url: value.wiki_url,
            tags: value.tags,
            entity_type: IndexedEntityType::Ability,
        }
    }
}
