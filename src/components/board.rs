use leptos::{logging::log, *};

use crate::game::{get_turn_outcome, Outcome, Token};

#[component]
pub fn Cell(
    current_player: ReadSignal<Token>,
    token: ReadSignal<Token>,
    set_token: WriteSignal<Token>,
) -> impl IntoView {
    let is_disabled = move || !matches!(token(), Token::Empty);

    view! {
        <button
            class="cell"
            on:click=move |_| {
                set_token(current_player());
            }

            disabled=is_disabled
        >

            "Click me: "
            {move || token().to_string()}
        </button>
    }
}

#[component]
pub fn Board<F>(player: ReadSignal<Token>, change_turn: F) -> impl IntoView
where
    F: Fn() + 'static,
{
    let tokens = (0..9)
        .map(|_| create_signal(Token::default()))
        .collect::<Vec<_>>();

    let outcome = {
        let tokens = tokens.clone();

        move || {
            let vals = tokens
                .iter()
                .map(|(token, _)| token())
                .collect::<Vec<_>>();

            match get_turn_outcome(&vals) {
                Outcome::Draw => "Draw".to_string(),
                Outcome::Win => format!("{} wins!", player()),
                Outcome::Continue => {
                    change_turn();
                    format!("{}'s turn", player())
                }
            }
        }
    };

    view! {
        <h2>{outcome}</h2>
        <ul class="board">
            {tokens
                .into_iter()
                .map(|(token, set_token)| {
                    view! {
                        <li>
                            <Cell current_player=player token set_token/>
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}
