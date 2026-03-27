use crate::pages::counter::Counter;
use yew::prelude::*; // Brings in Html, html! macro, FunctionComponent

#[function_component(Blog)]
pub fn blog() -> Html {
    html! {
        <main>
            <h1>{ "Blog Page" }</h1>
            <p>{ "This is where we count blog posts." }</p>
            <div class="counter-card">
            <Counter />
            </div>
        </main>
    }
}
