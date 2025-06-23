use serde::{Deserialize, Serialize};

fn default_filter_logic() -> String {
    String::from("or")
}

// fn default_page() -> usize {
//     1
// }
//
// fn default_per_page() -> usize {
//     20
// }

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct FilterParams {
    #[serde(default)]
    pub(crate) tags: Vec<String>, // comma-separated list
    #[serde(default = "default_filter_logic")]
    pub(crate) filter_logic: String, // "and" or "or"
}
