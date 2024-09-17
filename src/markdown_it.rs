use axum::response::Result;

pub async fn render(input: &str) -> Result<String> {
	let parser = &mut markdown_it::MarkdownIt::new();
	markdown_it::plugins::cmark::add(parser);
	markdown_it::plugins::extra::add(parser);

	let ast  = parser.parse(input);
	Ok(ast.render())
}
