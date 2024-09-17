use axum::response::Result;

pub async fn render(input: &str) -> Result<String> {
	Ok(markdown::to_html(input))
}
