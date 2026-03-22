use yew::prelude::*;
use gloo_net::http::Request;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;

#[derive(Serialize)]
struct RegisterPayload {
    name: String,
    email: String,
    password: String,
}

#[function_component(Register)]
pub fn register() -> Html {

    let name = use_state(|| "".to_string());
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let message = use_state(|| "".to_string());

    let on_name = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let on_email = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_password = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_submit = {
        let name = name.clone();
        let email = email.clone();
        let password = password.clone();
        let message = message.clone();

        Callback::from(move |_| {

            let payload = RegisterPayload {
                name: (*name).clone(),
                email: (*email).clone(),
                password: (*password).clone(),
            };

            let name_handle = name.clone();
            let email_handle = email.clone();
            let password_handle = password.clone();
            let message_handle = message.clone();

            spawn_local(async move {

                let response = Request::post("/api/register")
                    .header("Content-Type", "application/json")
                    .json(&payload)
                    .unwrap()
                    .send()
                    .await;

                match response {
                    Ok(resp) => {
                        if resp.status() == 200 {

                            message_handle.set("User registered successfully".to_string());

                            // clear the form
                            name_handle.set(String::new());
                            email_handle.set(String::new());
                            password_handle.set(String::new());

                        } else {
                            message_handle.set(format!("Server returned {}", resp.status()));
                        }
                    }
                    Err(_) => {
                        message_handle.set("Failed to contact server".to_string());
                    }
                }

            });

        })
    };

    html! {
        <div>

            <h1>{"Register"}</h1>

            <form autocomplete="off">

                <input
                    autocomplete="off"
                    placeholder="Name"
                    value={(*name).clone()}
                    oninput={on_name}
                />

                <input
                    autocomplete="off"
                    placeholder="Email"
                    value={(*email).clone()}
                    oninput={on_email}
                />

                <input
                    type="password"
                    autocomplete="new-password"
                    placeholder="Password"
                    value={(*password).clone()}
                    oninput={on_password}
                />

                <button type="button" onclick={on_submit}>
                    {"Register"}
                </button>

            </form>

            <p>{ (*message).clone() }</p>

        </div>
    }
}