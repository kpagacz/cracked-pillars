mod index_abilities;
mod index_items;
use std::collections::HashMap;

pub(crate) use index_abilities::index_abilities;
pub(crate) use index_items::index_items;

pub(crate) type Index = HashMap<String, Vec<i64>>;
