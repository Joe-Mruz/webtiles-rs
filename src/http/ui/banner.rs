//! Port of `assets/templates/banner.html` (now deleted).

use leptos::prelude::*;

#[component]
pub fn Banner(username: Option<String>) -> impl IntoView {
    view! {
        "Welcome to WebTiles!"
        {username.map(|username| view! { <br/> "Hello, " {username} "!" }) }
    }
}
