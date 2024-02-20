use crate::components::board::Board;
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="container">

            <h1>Leptos tac toe!</h1>

            <Board/>

        </div>
    }
}
