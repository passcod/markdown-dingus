use axum::response::Result;

pub async fn render(input: &str) -> Result<String> {
	let mut options = comrak::Options::default();
	options.render.r#unsafe = true;

	Ok(comrak::markdown_to_html(input, &options))
}
