use axum::{routing::get, Router, extract::Query, response::Result, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct TextQuery {
    text: String,
}

#[derive(Serialize)]
struct Response {
    name: &'static str,
    version: &'static str,
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
    version: &'static str,
    repo: &'static str,
}

macro_rules! renderers {
    ($($apiname:expr, $modname:ident, $version:literal, $cmark:literal, $repo:literal);+) => {
        $(
            mod $modname;
            async fn $modname(Query(TextQuery { text }): Query<TextQuery>) -> Result<Json<Response>> {
                if text.is_empty() || text.len() > 1000 {
                    return Err("text must be between 1 and 1000 characters".into());
                }

                Ok(Json(Response {
                    name: $apiname,
                    version: $version,
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
                        version: $version,
                        repo: $repo,
                    }),+
                ],
            })
        }

        #[shuttle_runtime::main]
        async fn main() -> shuttle_axum::ShuttleAxum {
            let router = Router::new()
                .route("/", get(about))
                .route("/registry", get(registry))
            $(
                .route(concat!("/", $apiname), get($modname))
            )+
            ;

            Ok(router.into())
        }
    };
}

renderers! {
    "pulldown-cmark", pulldown, "0.12.1", true, "https://github.com/pulldown-cmark/pulldown-cmark";
    "markdown", markdown, "0.3.0", false, "https://github.com/wooorm/markdown-rs"
}
