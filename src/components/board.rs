use leptos::*;

use crate::game::{get_turn_outcome, Outcome, Token};

#[component]
pub fn Cell(
    current_player: ReadSignal<Token>,
    token: ReadSignal<Token>,
    set_token: WriteSignal<Token>,
    outcome: Signal<Outcome>,
) -> impl IntoView {
    let is_disabled = move || outcome() != Outcome::Continue || !matches!(token(), Token::Empty);

    view! {
        <button
            class="cell"
            on:click=move |_| {
                set_token(current_player());
            }

            disabled=is_disabled
        >
            {move || match token() {
                Token::Empty => "Play here".to_string(),
                _ => token().to_string(),
            }}

        </button>
    }
}

#[component]
pub fn Board() -> impl IntoView {
    let (player, set_player) = create_signal(Token::default());

    let change_turn = move || {
        set_player.update(|player| match player {
            Token::X => *player = Token::O,
            _ => *player = Token::X,
        })
    };
    let tokens = (0..9)
        .map(|_| create_signal(Token::default()))
        .collect::<Vec<_>>();

    let outcome = Signal::derive({
        let tokens = tokens.clone();

        move || {
            let vals = tokens
                .iter()
                .map(|(token, _)| token())
                .collect::<Vec<_>>();

            get_turn_outcome(&vals)
        }
    });

    view! {
        <h2>
            {move || match outcome() {
                Outcome::Draw => "Draw".to_string(),
                Outcome::Win => format!("{} wins!", player()),
                Outcome::Continue => {
                    change_turn();
                    format!("{}'s turn", player())
                }
            }}

        </h2>
        <ul class="board">
            {tokens
                .into_iter()
                .map(|(token, set_token)| {
                    view! {
                        <li>
                            <Cell current_player=player token set_token outcome/>
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}
