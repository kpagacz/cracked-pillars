use std::sync::Arc;

use axum::{Json, extract::Query};
use serde::{Deserialize, Serialize};

use crate::backend::ability::Ability;

#[derive(Debug, Deserialize)]
pub(super) struct FilterParams {
    tags: Option<String>,         // comma-separated list
    filter_logic: Option<String>, // "and" or "or"
    page: Option<usize>,
    per_page: Option<usize>,
}

#[derive(Debug, Serialize)]
pub(super) struct PaginatedResponse<T> {
    data: Vec<T>,
    total: usize,
    page: usize,
    per_page: usize,
    total_pages: usize,
}

pub(super) async fn filter_abilities(
    Query(params): Query<FilterParams>,
    abilities: Arc<Vec<Ability>>,
    index: Arc<std::collections::HashMap<String, Vec<usize>>>,
) -> Json<PaginatedResponse<Ability>> {
    println!("TRACE: query params: {params:?}");
    // Parse comma-separated tags
    let tags: Vec<String> = params
        .tags
        .map(|t| t.split(',').map(String::from).collect())
        .unwrap_or_default();

    // Get filtered indices using the index
    let filtered_indices = match params.filter_logic.as_deref() {
        Some("and") => {
            // Intersect all tag indices
            tags.iter()
                .filter_map(|tag| index.get(tag))
                .fold(None, |acc, curr| match acc {
                    None => Some(curr.clone()),
                    Some(acc) => Some(intersect_sorted_vecs(&acc, curr)),
                })
                .unwrap_or_default()
        }
        Some("or") => {
            // Union all tag indices
            tags.iter()
                .filter_map(|tag| index.get(tag))
                .fold(None, |acc, curr| match acc {
                    None => Some(curr.clone()),
                    Some(acc) => Some(union_sorted_vecs(&acc, curr)),
                })
                .unwrap_or_default()
        }
        _ => vec![], // or return error
    };

    // Apply pagination
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);
    let start = (page - 1) * per_page;
    let end = start + per_page;

    // Map indices back to abilities
    let abilities = filtered_indices[start..end.min(filtered_indices.len())]
        .iter()
        .map(|&idx| abilities[idx].clone())
        .collect::<Vec<_>>();

    Json(PaginatedResponse {
        data: abilities,
        total: filtered_indices.len(),
        page,
        per_page,
        total_pages: filtered_indices.len().div_ceil(per_page),
    })
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
