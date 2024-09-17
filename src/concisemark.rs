use axum::response::Result;

pub async fn render(input: &str) -> Result<String> {
    Ok(concisemark::Page::new(input).render())
}