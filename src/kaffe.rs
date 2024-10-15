use axum::response::Result;
use kaffe::parser::{parse_markdown, generate_html};

pub async fn render(input: &str) -> Result<String> {
	let ast = parse_markdown(input).map_err(|err| err.to_string())?;
	let (html, _, _) = generate_html(&ast).await.map_err(|err| err.to_string())?;
	Ok(html)
}
