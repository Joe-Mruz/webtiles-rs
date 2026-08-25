//! Compile-time embedded static UI assets for webserver-rs's own lobby
//! page (`assets/static/`), so the server is a self-contained binary with
//! no runtime dependency on `crawl-ref/source/webserver` (the separate,
//! unrelated Python implementation). The lobby *markup* itself is Leptos
//! SSR (see `http/ui/`) rather than an embedded template file; this module
//! only covers `/static/*` (CSS, fonts, and the JS files below).
//!
//! `assets/static/scripts/{comm,client,chat,key_conversion,app,linkify}.js`
//! and `scripts/contrib/*` are untouched and must stay that way: the
//! external, per-crawl-version `game.html`/`game_data/static/game.js`
//! (reported via `client_path`, see `game/launch.rs`) requires these exact
//! AMD modules/APIs (confirmed against a real crawl-ref checkout - e.g.
//! `game.js` does `define(["jquery","exports","comm","client",
//! "key_conversion",...])` and reaches into their exports directly).
//! The per-game-version `game.html`/tiles/sounds a running `crawl` binary
//! reports via `client_path` are a different, inherently external and
//! disk-based concern (see `game/launch.rs`, `http/game_data.rs`) and are
//! untouched by this module.

use axum::extract::Path as AxumPath;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/static/"]
pub struct StaticFiles;

/// `GET /static/{*file}`. Lookups are HashMap key lookups against the
/// embedded set, not filesystem paths, so `..` traversal is not possible.
pub async fn serve_static(AxumPath(file): AxumPath<String>) -> impl IntoResponse {
    match StaticFiles::get(&file) {
        Some(asset) => {
            let mime = mime_guess::from_path(&file).first_or_octet_stream();
            let mut response = (StatusCode::OK, asset.data.into_owned()).into_response();
            response
                .headers_mut()
                .insert(header::CONTENT_TYPE, mime.as_ref().parse().unwrap());
            response
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
