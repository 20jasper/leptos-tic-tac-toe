use crate::components::board::{Board, Token};
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let (player, set_player) = create_signal(Token::X);
    view! {
		<div class="container">

			<picture>
				<source
					srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
					media="(prefers-color-scheme: dark)"
				/>
				<img
					src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
					alt="Leptos Logo"
					height="200"
					width="400"
				/>
			</picture>

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
