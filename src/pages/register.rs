use yew::prelude::*;
use gloo_net::http::Request;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;

#[cfg(debug_assertions)]
const API_BASE: &str = "/api";

#[cfg(not(debug_assertions))]
const API_BASE: &str = "https://rustbackend-production.up.railway.app";

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

    // Clear message when typing
    let on_name = {
        let name = name.clone();
        let message = message.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
            message.set("".to_string());
        })
    };

    let on_email = {
        let email = email.clone();
        let message = message.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
            message.set("".to_string());
        })
    };

    let on_password = {
        let password = password.clone();
        let message = message.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
            message.set("".to_string());
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

                let response = Request::post(&format!("{}/register", API_BASE))
                    .header("Content-Type", "application/json")
                    .json(&payload)
                    .unwrap()
                    .send()
                    .await;

                match response {
                    Ok(resp) => {
                        if resp.status() == 200 {
                            message_handle.set("User registered successfully".to_string());

                            // clear form
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
        <div class="page-content">
            <h1>{ "Register" }</h1>

            <form autocomplete="off">

                <div class="form-row">
                    <label>{ "Name:" }</label>
                    <input
                        type="text"
                        name="random_name"
                        placeholder="Enter your name"
                        value={(*name).clone()}
                        oninput={on_name}
                        autocomplete="off"
                    />
                </div>

                <div class="form-row">
                    <label>{ "Email address:" }</label>
                    <input
                        type="email"
                        name="random_email"
                        placeholder="Enter your email"
                        value={(*email).clone()}
                        oninput={on_email}
                        autocomplete="off"
                    />
                </div>

                <div class="form-row">
                    <label>{ "Password:" }</label>
                    <input
                        type="password"
                        name="new-password"
                        placeholder="Enter your password"
                        value={(*password).clone()}
                        oninput={on_password}
                        autocomplete="new-password"
                    />
                </div>

                <button type="button" onclick={on_submit}>
                    { "Register" }
                </button>

            </form>

            <p>{ (*message).clone() }</p>
        </div>
    }
}
