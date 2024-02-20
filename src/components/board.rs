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
pub fn Cell(player: ReadSignal<Token>) -> impl IntoView {
    let (token, set_token) = create_signal(Token::default());
    let is_disabled = create_memo(move |_| !matches!(token.get(), Token::Empty));

    view! {
        <button
            on:click= move |_| {
                set_token(player())
            }
            disabled= is_disabled()
        >
            "Click me: " {move || token.get().to_string()}
        </button>
    }
}
