use std::fmt::Display;

use leptos::*;

#[derive(Debug, Clone, Default, Copy)]
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
pub fn Cell(
    current_player: ReadSignal<Token>,
    token: ReadSignal<Token>,
    set_token: WriteSignal<Token>,
) -> impl IntoView {
    let is_disabled = move || !matches!(token(), Token::Empty);

    view! {
        <button class="cell" on:click=move |_| { set_token(current_player()) } disabled=is_disabled>

            "Click me: "
            {move || token().to_string()}
        </button>
    }
}

#[component]
pub fn Board(player: ReadSignal<Token>) -> impl IntoView {
    let tokens = (0..9)
        .map(|_| create_signal(Token::default()))
        .collect::<Vec<_>>();

    let count = {
        let tokens = tokens.clone();

        move || {
            tokens
                .iter()
                .filter(|(token, _)| matches!(token(), Token::Empty))
                .count()
        }
    };

    view! {
        <h1>{count}</h1>
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
