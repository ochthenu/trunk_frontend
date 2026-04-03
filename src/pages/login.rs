use yew::prelude::*;
use gloo::net::http::Request;
use gloo::storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};

#[cfg(debug_assertions)]
const API_BASE: &str = "/api";

#[cfg(not(debug_assertions))]
const API_BASE: &str = "https://rustbackend-production.up.railway.app";

#[derive(Serialize)]
struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
}

#[function_component(Login)]
pub fn login() -> Html {
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let message = use_state(|| "".to_string());

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
        let email = email.clone();
        let password = password.clone();
        let message = message.clone();

        Callback::from(move |_| {
            let payload = LoginPayload {
                email: (*email).clone(),
                password: (*password).clone(),
            };

            let message_handle = message.clone();

            spawn_local(async move {
                let response = Request::post(&format!("{}/login", API_BASE))
                    .header("Content-Type", "application/json")
                    .json(&payload)
                    .unwrap()
                    .send()
                    .await;

                match response {
                    Ok(resp) => {
                        if resp.status() == 200 {
                            match resp.json::<LoginResponse>().await {
                                Ok(data) => {
                                    // ✅ store token
                                    LocalStorage::set("token", data.token).unwrap();
                                    message_handle.set("Login successful".to_string());
                                }
                                Err(_) => {
                                    message_handle.set("Invalid response".to_string());
                                }
                            }
                        } else {
                            message_handle.set("Invalid email or password".to_string());
                        }
                    }
                    Err(_) => {
                        message_handle.set("Server error".to_string());
                    }
                }
            });
        })
    };

    html! {
        <div>
            <h1>{ "Login" }</h1>

            <form autocomplete="off">

                <input
                    type="email"
                    placeholder="Email"
                    value={(*email).clone()}
                    oninput={on_email}
                />

                <input
                    type="password"
                    placeholder="Password"
                    value={(*password).clone()}
                    oninput={on_password}
                />

                <button type="button" onclick={on_submit}>
                    { "Login" }
                </button>

            </form>

            <p>{ (*message).clone() }</p>
        </div>
    }
}