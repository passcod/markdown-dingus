use axum::response::Result;

pub async fn render(input: &str) -> Result<String> {
    let parser = pulldown_cmark::Parser::new(input);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    Ok(html_output)
}