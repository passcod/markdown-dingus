use axum::response::Result;

pub async fn render(input: &str) -> Result<String> {
	Ok(mini_markdown::render(input))
}
