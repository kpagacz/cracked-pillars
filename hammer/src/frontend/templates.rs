use std::sync::Arc;
use tera::Tera;
use axum::{
    response::{Html, IntoResponse},
    extract::State,
};
use tracing::error;

// Create a type alias for our template state
pub type TemplateState = Arc<Tera>;

// Initialize Tera with our templates
pub fn init_templates() -> Result<TemplateState, tera::Error> {
    let tera = Tera::new("src/frontend/templates/**/*")?;
    Ok(Arc::new(tera))
}

// Helper function to render a template with a context
pub async fn render_template(
    State(templates): State<TemplateState>,
    template_name: &str,
    context: tera::Context,
) -> impl IntoResponse {
    match templates.render(template_name, &context) {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            error!("Template error: {}", e);
            Html(format!("Template error: {}", e)).into_response()
        }
    }
}

// Helper function to create a new context
pub fn create_context() -> tera::Context {
    tera::Context::new()
}
