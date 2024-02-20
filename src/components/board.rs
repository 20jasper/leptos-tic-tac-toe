use std::fmt::Display;

use leptos::{logging::log, *};
use web_sys::MouseEvent;

use crate::game::{get_turn_outcome, Outcome};

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
pub enum Token {
    X,
    O,
    #[default]
    Empty,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Token::X => "X",
            Token::O => "O",
            Token::Empty => "Empty",
        };

        write!(f, "{}", s)
    }
}

#[component]
pub fn Cell<F: Fn(MouseEvent) + 'static>(
    current_player: ReadSignal<Token>,
    change_turn: F,
    token: ReadSignal<Token>,
    set_token: WriteSignal<Token>,
) -> impl IntoView {
    let is_disabled = move || !matches!(token(), Token::Empty);

    view! {
        <button
            class="cell"
            on:click=move |e| {
                set_token(current_player());
                change_turn(e)
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
    F: Fn(MouseEvent) + 'static + Clone + Copy,
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

            log!("vals: {:?}", vals);
            let outcome = get_turn_outcome(&vals);
            match outcome {
                Outcome::Draw => "Draw".to_string(),
                Outcome::Win => format!("{} wins!", player()),
                Outcome::Continue => format!("{}'s turn", player()),
            }
        }
    };

    view! {
        <h3>{outcome}</h3>
        <ul class="board">
            {tokens
                .into_iter()
                .map(|(token, set_token)| {
                    view! {
                        <li>
                            <Cell current_player=player change_turn token set_token/>
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}
