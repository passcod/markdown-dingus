use axum::response::Result;
use kaffe::parser::{parse_markdown, generate_html};

pub async fn render(input: &str) -> Result<String> {
	// as of kaffe 2ed231a there's a bug where it won't parse the last item of input;
	// so if we add some newlines and an underscore, it will skip that but render the
	// desired content.
	let hack = format!("{input}\n\n_");

	let ast = parse_markdown(&hack).map_err(|err| err.to_string())?;
	let (html, _, _) = generate_html(&dbg!(ast)).await.map_err(|err| err.to_string())?;
	Ok(html)
}
