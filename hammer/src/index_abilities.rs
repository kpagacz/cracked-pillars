use crate::models::Ability;
use std::collections::HashMap;

pub(crate) fn index_abilities(abilities: &[Ability]) -> HashMap<String, Vec<usize>> {
    let mut index: HashMap<String, Vec<usize>> = HashMap::new();
    for (id, ability) in abilities.iter().enumerate() {
        for tag in ability.effects.iter().flat_map(|effect| &effect.tags) {
            index
                .entry(tag.clone())
                .and_modify(|abilities| abilities.push(id))
                .or_default();
        }
    }
    index.values_mut().for_each(|list| {
        list.sort();
        list.dedup();
    });
    index
}
