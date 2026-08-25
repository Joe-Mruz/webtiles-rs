//! Port of the hand-rolled "Play now:" links HTML built in
//! `crate::websocket::connection::send_game_links` (`ServerMessage::SetGameLinks`).

use leptos::prelude::*;

#[component]
pub fn GameLinks(games: Vec<(String, String)>) -> impl IntoView {
    view! {
        "Play now:"
        {games
            .into_iter()
            .map(|(id, name)| view! {
                <br/>
                <a href=format!("#play-{id}")>{name}</a>
                " ("
                <a class="edit_rc_link" data-game_id=id.clone()>"edit rc"</a>
                ")"
            })
            .collect_view()}
    }
}
