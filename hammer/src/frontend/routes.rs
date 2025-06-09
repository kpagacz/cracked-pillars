use crate::backend::load_abilities::read_abilities;
use crate::frontend::{TemplateState, create_context, render_template};
use axum::{Router, extract::State, routing::get};
use chrono::Datelike;
use std::sync::Arc;

pub fn get_frontend_routes(templates: TemplateState) -> Router<()> {
    Router::new()
        .route(
            "/",
            get({
                let templates = Arc::clone(&templates);
                move || async move {
                    let mut context = create_context();
                    context.insert("title", "Cracked Pillars");
                    context.insert("year", &chrono::Utc::now().year());
                    render_template(State(templates), "home.html", context).await
                }
            }),
        )
        .route(
            "/abilities",
            get({
                let templates = Arc::clone(&templates);
                move || async move {
                    let mut context = create_context();
                    context.insert("title", "Abilities");
                    context.insert("year", &chrono::Utc::now().year());

                    // Load available tags from abilities' effects
                    let abilities = read_abilities().expect("Failed to load abilities");
                    let mut tags: Vec<String> = abilities
                        .iter()
                        .flat_map(|ability| ability.effects.iter())
                        .flat_map(|effect| effect.tags.clone())
                        .collect::<std::collections::HashSet<_>>()
                        .into_iter()
                        .collect();
                    tags.sort();

                    context.insert("abilities", &abilities);
                    context.insert("tags", &tags);
                    render_template(State(templates), "abilities/list.html", context).await
                }
            }),
        )
        .with_state(templates)
}
