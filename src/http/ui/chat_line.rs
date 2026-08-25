//! Port of the hand-rolled chat line HTML built in
//! `crate::websocket::connection::chat` (`ServerMessage::Chat`).

use leptos::prelude::*;

#[component]
pub fn ChatLine(sender: String, text: String) -> impl IntoView {
    view! {
        <span class="chat_sender">{sender}</span>
        ": "
        <span class="chat_msg">{text}</span>
    }
}
