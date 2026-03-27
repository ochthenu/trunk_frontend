use yew::prelude::*;

#[function_component(Counter)]
pub fn counter() -> Html {
    // State for the counter
    let count = use_state(|| 0);

    // Increment handler
    let onclick = {
        let count = count.clone();
        Callback::from(move |_| count.set(*count + 1))
    };

    html! {
        <div class="counter">
            <p>{ format!("Count: {}", *count) }</p>
            <button {onclick}>{ "Increment" }</button>
        </div>
    }
}
