use web_sys::{HtmlInputElement, SubmitEvent};
use yew::prelude::*;

use gloo::net::http::Request;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;

#[derive(Serialize)]
struct ContactPayload {
    name: String,
    email: String,
}

#[function_component(Contact)]
pub fn contact() -> Html {
    let name = use_state(|| String::new());
    let email = use_state(|| String::new());
    let submitted = use_state(|| false);

    let on_name_input = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let on_email_input = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_submit = {
        let name = name.clone();
        let email = email.clone();
        let submitted = submitted.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let payload = ContactPayload {
                name: (*name).clone(),
                email: (*email).clone(),
            };

            let submitted = submitted.clone();

            spawn_local(async move {
                let _ = Request::post("https://formspree.io/f/xwvnvbgp")
                    .header("Accept", "application/json")
                    .json(&payload)
                    .unwrap()
                    .send()
                    .await;

                submitted.set(true);
            });
        })
    };

    html! {
        <main class="contact-card">
            <h1>{ "Contact Us" }</h1>

            <form onsubmit={on_submit}>
                <input
                    type="text"
                    placeholder="Your Name"
                    value={(*name).clone()}
                    oninput={on_name_input}
                    required=true
                />
                <input
                    type="email"
                    placeholder="Your Email"
                    value={(*email).clone()}
                    oninput={on_email_input}
                    required=true
                />
                <button type="submit">{ "Submit" }</button>
            </form>

            if *submitted {
                <p>
                    { format!("Thank you, {}! We’ve sent you an email.", *name) }
                </p>
            }
        </main>
    }
}


