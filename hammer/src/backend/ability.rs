use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) enum Activation {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "modal")]
    Modal,
    #[serde(rename = "passive")]
    Passive,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct Effect {
    pub(crate) targets: Vec<Target>,
    pub(crate) condition: Option<String>,
    pub(crate) value: String,
    pub(crate) duration: Option<f32>,
    pub(crate) tags: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) enum Target {
    #[serde(rename = "self")]
    _Self,
    #[serde(rename = "allied_aoe")]
    AlliedAoe,
    #[serde(rename = "hazard_aoe")]
    HazardAoe,
    #[serde(rename = "foe_aoe")]
    FoeAoe,
    #[serde(rename = "foe_target")]
    Foe,
    #[serde(rename = "attackers")]
    Attackers,
    #[serde(rename = "target")]
    Any,
    #[serde(rename = "jump_targets")]
    JumpTargets,
    #[serde(rename = "summon")]
    Summon,
    #[serde(rename = "target+beam")]
    AnyAndBeam,
    #[serde(rename = "friendly_target")]
    Friendly,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct Origin {
    #[serde(rename = "type")]
    pub(crate) _type: String,
    pub(crate) value: OriginValue,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub(crate) enum OriginValue {
    Progression(ProgressionOrigin),
    Item(String),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct ProgressionOrigin {
    pub(crate) class: String,
    pub(crate) level: i32,
    pub(crate) learn_level: LearnLevel,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub(crate) enum LearnLevel {
    Numeric(i32),
    Text(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Ability {
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) description: String,
    pub(crate) activation: Activation,
    pub(crate) area_of_effect: Option<String>,
    pub(crate) notes: Option<Vec<String>>,
    pub(crate) effects: Vec<Effect>,
    pub(crate) origin: Origin,
    pub(crate) keywords: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let json = r#"
        {
  "name": "Citzal's Martial Power",
  "url": "https://pillarsofeternity.fandom.com/wiki/Citzal%27s_Martial_Power_(Deadfire)",
  "description": "Caster temporarily sacrifices arcane power for martial might. The wizard gains bonuses to Deflection and Accuracy, as well as the Strong, Fit, and Quick Inspirations, but becomes unable to cast spells for the duration of the effect.",
  "activation": "active",
  "area_of_effect": "Self",
  "notes": [
    "This spell is part of the Conjurer subclass spell list",
    "The spell cannot be cast while under the effects of Arcane Dampener"
  ],
  "effects": [
    {
      "targets": ["self"],
      "value": "Strong, Fit, Quick, +20 Deflection, +20 Accuracy, +1 enemies Engaged",
      "duration": 30.0,
      "tags": ["strong", "fit", "quick", "mod_deflection", "mod_accuracy", "engagement_slots"]
    },
    {
      "targets": ["self"],
      "value": "Spellcasting Disabled",
      "duration": 30.0,
      "tags": ["spellcasting_disabled"]
    }
  ],
  "origin": {
    "type": "progression",
    "value": {
      "class": "Wizard",
      "learn_level": 13,
      "level": 7
    }
  },
  "keywords": ["Enchanting"]
}
        "#;
        let ability: Ability = serde_json::from_str(json).unwrap();
        println!("{ability:?}");
    }
}
