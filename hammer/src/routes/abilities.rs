use crate::pagination::PaginatedResponse;
use axum::Json;
use axum_extra::extract::Query;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;

use crate::abbreviated_ability::AbbreviatedAbility;
use crate::ability::Ability;

fn default_filter_logic() -> String {
    String::from("or")
}

fn default_page() -> usize {
    1
}

fn default_per_page() -> usize {
    20
}

fn default_abbreviated() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct FilterParams {
    #[serde(default)]
    tags: Vec<String>, // comma-separated list
    #[serde(default = "default_filter_logic")]
    filter_logic: String, // "and" or "or"
    #[serde(default = "default_page")]
    page: usize,
    #[serde(default = "default_per_page")]
    per_page: usize,
    #[serde(default = "default_abbreviated")]
    abbreviated: bool,
}

pub(crate) async fn get(
    Query(params): Query<FilterParams>,
    abilities: Arc<Vec<Ability>>,
    index: Arc<HashMap<String, Vec<usize>>>,
) -> Json<Value> {
    tracing::event!(tracing::Level::DEBUG, "query params: {params:?}");
    // Parse comma-separated tags
    let tags: Vec<String> = params.tags;

    // Get filtered indices using the index
    let filtered_indices: Vec<usize> = match params.filter_logic.as_str() {
        "or" => {
            tracing::event!(
                tracing::Level::TRACE,
                "Inside the or filtered indices match arm"
            );
            tags.iter()
                .filter_map(|tag| index.get(tag))
                .fold(Vec::new(), |acc, curr| union_sorted_vecs(&acc, curr))
        }
        "and" => {
            tracing::event!(
                tracing::Level::TRACE,
                "Inside the and filtered indices match arm"
            );
            let all_abilities = (0..abilities.len()).collect();
            tags.iter()
                .filter_map(|tag| index.get(tag))
                .fold(all_abilities, |acc, curr| {
                    tracing::event!(
                        tracing::Level::TRACE,
                        "One of the indices vectors: {curr:?}"
                    );
                    intersect_sorted_vecs(&acc, curr)
                })
        }
        _ => vec![],
    };
    tracing::event!(
        tracing::Level::TRACE,
        "filtered indices: {filtered_indices:?}"
    );

    // Apply pagination
    let page = params.page;
    let per_page = params.per_page;
    let start = (page - 1) * per_page;
    let end = start + per_page;

    let abilities = filtered_indices[start..end.min(filtered_indices.len())]
        .iter()
        .map(|&idx| abilities[idx].clone());
    // Map indices back to abilities
    if params.abbreviated {
        let abilities = abilities.map(AbbreviatedAbility::from).collect();
        Json(json!(PaginatedResponse {
            data: abilities,
            total: filtered_indices.len(),
            page,
            per_page,
            total_pages: filtered_indices.len().div_ceil(per_page),
        }))
    } else {
        let abilities = abilities.collect();

        Json(json!(PaginatedResponse {
            data: abilities,
            total: filtered_indices.len(),
            page,
            per_page,
            total_pages: filtered_indices.len().div_ceil(per_page),
        }))
    }
}

// Helper function to intersect two sorted vectors
fn intersect_sorted_vecs(a: &[usize], b: &[usize]) -> Vec<usize> {
    let mut result = Vec::with_capacity(a.len().min(b.len()));
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                result.push(a[i]);
                i += 1;
                j += 1;
            }
        }
    }
    result
}

// Helper function to union two sorted vectors
fn union_sorted_vecs(a: &[usize], b: &[usize]) -> Vec<usize> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => {
                result.push(a[i]);
                i += 1;
            }
            std::cmp::Ordering::Greater => {
                result.push(b[j]);
                j += 1;
            }
            std::cmp::Ordering::Equal => {
                result.push(a[i]);
                i += 1;
                j += 1;
            }
        }
    }

    // Add remaining elements
    result.extend_from_slice(&a[i..]);
    result.extend_from_slice(&b[j..]);
    result
}
