use yew::prelude::*;
use yew_router::prelude::*;
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

    let navigator = use_navigator();

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
        let navigator = navigator.clone();

        Callback::from(move |_| {
            let payload = LoginPayload {
                email: (*email).clone(),
                password: (*password).clone(),
            };

            let message_handle = message.clone();
            let navigator = navigator.clone();

            spawn_local(async move {
                let response = Request::post(&format!("{}/login", API_BASE))
                    .header("Content-Type", "application/json")
                    .json(&payload);

                let response = match response {
                    Ok(req) => req.send().await,
                    Err(_) => {
                        message_handle.set("Request build failed".into());
                        return;
                    }
                };

                match response {
                    Ok(resp) => {
                        if resp.status() == 200 {
                            match resp.json::<LoginResponse>().await {
                                Ok(data) => {
                                    let _ = LocalStorage::set("token", data.token);
                                    message_handle.set("Login successful".to_string());

                                    if let Some(nav) = navigator {
                                        nav.push(&crate::app::Route::Users);
                                    }
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
        <div class="page-content">
            <h1>{ "Login" }</h1>

            <div class="form-row">
                <label>{ "Email address:" }</label>
                <input
                    type="email"
                    placeholder="Enter your email"
                    value={(*email).clone()}
                    oninput={on_email}
                />
            </div>

            <div class="form-row">
                <label>{ "Password:" }</label>
                <input
                    type="password"
                    placeholder="Enter your password"
                    value={(*password).clone()}
                    oninput={on_password}
                />
            </div>

            <button onclick={on_submit}>
                { "Login" }
            </button>

            <p>{ (*message).clone() }</p>
        </div>
    }
}