use axum::response::Result;

pub async fn render(input: &str) -> Result<String> {
	match markdowny::markdown2html(input) {
		Ok(html) => Ok(html),
		Err((_, e)) => Err(e.into()),
	}
}
