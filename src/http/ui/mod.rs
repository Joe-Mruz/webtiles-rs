//! Leptos SSR replacement for the old string-template lobby chrome (see
//! `../templates.rs`). SSR-only (feature `ssr`, no `hydrate`/`csr`) - these
//! components are rendered once per request via `leptos::ssr::render_to_string`
//! and never ship any WASM to the browser. The produced markup must keep the
//! exact ids/classes that `assets/static/scripts/{client,chat}.js` select on
//! (those files, and everything they in turn require, are untouched - see
//! module docs on `crate::http::assets` for why).

mod banner;
mod chat_line;
mod game_links;
mod lobby;

pub use banner::Banner;
pub use chat_line::ChatLine;
pub use game_links::GameLinks;
pub use lobby::LobbyPage;
