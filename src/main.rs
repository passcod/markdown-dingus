use std::{collections::HashMap, sync::LazyLock};

use axum::{extract::Query, response::Result, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use toml::Table;

mod log;

const CARGO_TOML: &str = include_str!("../Cargo.toml");

#[derive(Deserialize)]
struct CargoToml {
	dependencies: Table,
}

static VERSIONS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
	let CargoToml { dependencies } = toml::from_str(CARGO_TOML).unwrap();
	dependencies
		.into_iter()
		.filter_map(|(name, version)| match version {
			toml::Value::String(version) => Some((name, version)),
			toml::Value::Table(mut dep) => {
				if let Some(toml::Value::String(version)) = dep.remove("version") {
					Some((name, version))
				} else {
					None
				}
			}
			_ => None,
		})
		.collect()
});

#[derive(Deserialize)]
struct TextQuery {
	text: String,
}

#[derive(Serialize)]
struct Response {
	name: &'static str,
	version: String,
	html: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct RegistryEntry {
	lang: &'static str,
	name: &'static str,
	url: &'static str,
	common_mark: &'static str,
	repo: &'static str,
}

#[derive(Serialize)]
struct About {
	about: &'static str,
	version: &'static str,
	babelmark: &'static str,
	repo: &'static str,
	registry: &'static str,
	crates: Vec<Crate>,
}

#[derive(Serialize)]
struct Crate {
	name: &'static str,
	version: String,
	repo: &'static str,
}

macro_rules! renderers {
	($($apiname:expr, $modname:ident, $cmark:literal, $repo:literal);+) => {
		$(
			mod $modname;
			async fn $modname(Query(TextQuery { text }): Query<TextQuery>) -> Result<Json<Response>> {
				if text.is_empty() || text.len() > 1000 {
					return Err("text must be between 1 and 1000 characters".into());
				}

				Ok(Json(Response {
					name: $apiname,
					version: VERSIONS.get($apiname).map(ToString::to_string).unwrap_or_default(),
					html: $modname::render(&text).await?,
				}))
			}
		)+

		async fn registry() -> Json<Vec<RegistryEntry>> {
			Json(vec![
				$(RegistryEntry {
					lang: "Rust",
					name: $apiname,
					url: concat!("https://markdown-dingus.shuttleapp.rs/", $apiname),
					common_mark: stringify!($cmark),
					repo: $repo,
				}),+
			])
		}

		async fn about() -> Json<About> {
			Json(About {
				about: "Comparison of Markdown Renderers for Rust crates, for the Babelmark v3 service",
				version: env!("CARGO_PKG_VERSION"),
				babelmark: "https://babelmark.github.io/",
				repo: "https://github.com/passcod/markdown-dingus",
				registry: "https://markdown-dingus.shuttleapp.rs/registry",
				crates: vec![
					$(Crate {
						name: $apiname,
						version: VERSIONS.get($apiname).map(ToString::to_string).unwrap_or_default(),
						repo: $repo,
					}),+
				],
			})
		}

		#[shuttle_runtime::main]
		async fn main() -> shuttle_axum::ShuttleAxum {
			LazyLock::force(&VERSIONS);

			let router = Router::new()
				.route("/", get(about))
				.route("/registry", get(registry))
			$(
				.route(concat!("/", $apiname), get($modname))
			)+
			.layer(log::logging_layer());

			Ok(router.into())
		}
	};
}

renderers! {
	"pulldown-cmark", pulldown, true, "https://github.com/pulldown-cmark/pulldown-cmark";
	"markdown", markdown, true, "https://github.com/wooorm/markdown-rs";
	"comrak", comrak, true, "https://github.com/kivikakk/comrak";
	"markdown-it", markdown_it, true, "https://github.com/markdown-it-rust/markdown-it";
	"markdowny", markdowny, false, "https://gitlab.com/bitpowder/indigo-ng";
	"concisemark", concisemark, false, "https://github.com/ikey4u/concisemark";
	"mdxt", mdxt, false, "https://github.com/baehyunsol/mdxt";
	"mini_markdown", mini_markdown, false, "https://github.com/darakian/mini_markdown";
	//"kaffe", kaffe, false, "https://github.com/Schachte/kaffe-rs"
}
