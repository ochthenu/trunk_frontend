use yew::prelude::*;
use gloo::net::http::Request;
use gloo::storage::{LocalStorage, Storage}; // ✅ FIXED HERE
use wasm_bindgen_futures::spawn_local;

#[cfg(debug_assertions)]
const API_BASE: &str = "/api";

#[cfg(not(debug_assertions))]
const API_BASE: &str = "https://rustbackend-production.up.railway.app";

#[derive(Clone, PartialEq, serde::Deserialize, Debug)]
struct User {
    id: i32,
    name: String,
}

#[function_component(Users)]
pub fn users() -> Html {
    let users = use_state(|| Vec::<User>::new());

    // Get token safely
    let token: String = LocalStorage::get("token").unwrap_or_default();

    // Load users
    {
        let users = users.clone();
        let token = token.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {

                if token.is_empty() {
                    web_sys::console::log_1(&"❌ No token found".into());
                    return;
                }

                let url = format!("{}/users", API_BASE);
                web_sys::console::log_1(&format!("Fetching: {}", url).into());

                match Request::get(&url)
                    .header("Authorization", &format!("Bearer {}", token))
                    .send()
                    .await
                {
                    Ok(resp) => {
                        web_sys::console::log_1(&format!("Status: {}", resp.status()).into());

                        if resp.status() == 200 {
                            match resp.json::<Vec<User>>().await {
                                Ok(data) => users.set(data),
                                Err(e) => web_sys::console::log_1(&format!("JSON error: {:?}", e).into()),
                            }
                        } else {
                            web_sys::console::log_1(&"❌ Unauthorized or forbidden".into());
                        }
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Request error: {:?}", e).into());
                    }
                }
            });

            || ()
        });
    }

    html! {
        <div>
            <h1>{ "User Administration" }</h1>

            {
                if token.is_empty() {
                    html! { <p>{ "Please log in to view users." }</p> }
                } else {
                    html! {
                        <table class="users-table">
                            <thead>
                                <tr>
                                    <th>{ "Name" }</th>
                                    <th>{ "ID" }</th>
                                    <th>{ "Action" }</th>
                                </tr>
                            </thead>

                            <tbody>
                                {
                                    for users.iter().map(|user| {
                                        let id = user.id;
                                        let users = users.clone();
                                        let token = token.clone();

                                        let delete = Callback::from(move |_| {
                                            let users = users.clone();
                                            let token = token.clone();

                                            spawn_local(async move {

                                                let _ = Request::delete(
                                                    &format!("{}/users/{}", API_BASE, id)
                                                )
                                                .header("Authorization", &format!("Bearer {}", token))
                                                .send()
                                                .await;

                                                // reload users
                                                if let Ok(resp) = Request::get(&format!("{}/users", API_BASE))
                                                    .header("Authorization", &format!("Bearer {}", token))
                                                    .send()
                                                    .await
                                                {
                                                    if let Ok(data) = resp.json::<Vec<User>>().await {
                                                        users.set(data);
                                                    }
                                                }
                                            });
                                        });

                                        html! {
                                            <tr>
                                                <td>{ &user.name }</td>
                                                <td>{ user.id }</td>
                                                <td>
                                                    <button onclick={delete}>{ "Delete" }</button>
                                                </td>
                                            </tr>
                                        }
                                    })
                                }
                            </tbody>
                        </table>
                    }
                }
            }
        </div>
    }
}

