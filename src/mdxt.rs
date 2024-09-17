use axum::response::Result;

pub async fn render(input: &str) -> Result<String> {
    Ok(mdxt::render_to_html_with_default_options(input))
}