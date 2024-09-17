use axum::response::Result;

pub async fn render(input: &str) -> Result<String> {
    Ok(comrak::markdown_to_html(input, &Default::default()))
}