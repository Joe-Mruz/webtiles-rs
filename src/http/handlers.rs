//! HTTP handlers, matching `MainHandler`/`status.LobbyHandler`/
//! `status.VersionHandler` in the Python implementation.

use std::collections::HashMap;

use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::response::{Html, IntoResponse, Json};

use leptos::prelude::*;

use crate::http::ui::LobbyPage;
use crate::state::AppState;

/// `GET /`: renders the lobby page. See `../../ARCHITECTURE.md` §2 and
/// `http/ui/` (Leptos SSR, no hydration - see its module docs) for what
/// backs this.
pub async fn main_page(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let is_https = headers
        .get("x-forwarded-proto")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == "https")
        .unwrap_or(false);
    let protocol = if is_https { "wss://" } else { "ws://" };
    let host = headers
        .get(axum::http::header::HOST)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("localhost");
    let socket_server = format!("{protocol}{host}/socket");

    // NOT YET PORTED: full recovery-token lookup/expiry semantics
    // (userdb.rs does not yet implement the `recovery_tokens` table
    // queries `userdb.find_recovery_token` performs); this currently
    // always treats a present token as valid.
    let reset_token = params.get("ResetToken").cloned();

    let allow_password_reset = state.config.allow_password_reset;
    let admin_password_reset = state.config.admin_password_reset;
    let game_version = env!("CARGO_PKG_VERSION").to_string();

    let html = view! {
        <LobbyPage
            socket_server=socket_server
            game_version=game_version
            allow_password_reset=allow_password_reset
            admin_password_reset=admin_password_reset
            reset_token=reset_token
            reset_token_error=None
        />
    }
    .to_html();
    Html(format!("<!DOCTYPE HTML>\n{html}")).into_response()
}

/// `GET /status/version/`, matching `status.VersionHandler`. Field names
/// are intentionally renamed for the Rust implementation
/// (`tornado`->`axum`, `python`->`rust`) - documented as an intentional,
/// client-invisible difference in `PROTOCOL.md` §8.
pub async fn status_version() -> impl IntoResponse {
    Json(serde_json::json!({
        "webtiles": env!("CARGO_PKG_VERSION"),
        "axum": "0.8",
        "rust": rustc_version(),
        "rust_supported": true,
    }))
}

fn rustc_version() -> &'static str {
    option_env!("CARGO_PKG_RUST_VERSION").unwrap_or("unknown")
}

/// `GET /status/lobby/`, matching `status.LobbyHandler`.
///
/// Simplification: reuses the internal [`crate::protocol::LobbyEntry`]
/// shape (same one sent over the websocket `lobby_entry` message) rather
/// than reproducing Python's slightly different ad-hoc JSON schema for
/// this endpoint (`name`/`viewers`/`watch_url` plus flattened `v`, `vlong`,
/// `tiles`, `race`, `cls` fields not currently tracked here). Any external
/// dashboard consuming this endpoint's exact field names would need
/// updating - see `PROTOCOL.md` §8.
pub async fn status_lobby(State(state): State<AppState>) -> impl IntoResponse {
    Json(state.games.lobby_entries().await)
}
