use crate::components::board::{Board, Token};
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let (player, set_player) = create_signal(Token::X);

    let change_turn = move || {
        set_player.update(|player| match player {
            Token::X => *player = Token::O,
            _ => *player = Token::X,
        })
    };

    view! {
        <div class="container">

            <h1>"it's " {move || player().to_string()} "'s turn"</h1>

            <Board player change_turn/>

        </div>
    }
}
