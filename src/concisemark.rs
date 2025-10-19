use axum::response::Result;

pub async fn render(input: &str) -> Result<String> {
	let raw = concisemark::Page::new(input).render();

	let stripped = raw.strip_prefix("<div>").unwrap_or(&raw);
	let stripped = stripped.strip_suffix("</div>").unwrap_or(stripped);

	Ok(stripped.into())
}
