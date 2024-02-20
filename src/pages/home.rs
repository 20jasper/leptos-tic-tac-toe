use crate::components::board::{Board, Token};
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let (player, set_player) = create_signal(Token::X);
    view! {
        <div class="container">

            <h1>"it's " {move || player.get().to_string()} "'s turn"</h1>

            <button on:click=move |_| match player.get() {
                Token::X => set_player(Token::O),
                _ => set_player(Token::X),
            }>

                change player
            </button>
            <Board player=player/>

        </div>
    }
}
