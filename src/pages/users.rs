use yew::prelude::*;
use gloo::net::http::Request;
use gloo_storage::{LocalStorage, Storage};
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

    // ✅ Fetch users on load (WITH TOKEN)
    {
        let users = users.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {

                let token: String = LocalStorage::get("token").unwrap_or_default();

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
                                Ok(data) => {
                                    web_sys::console::log_1(&format!("Users: {:?}", data).into());
                                    users.set(data);
                                }
                                Err(e) => {
                                    web_sys::console::log_1(&format!("JSON error: {:?}", e).into());
                                }
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

                            let delete = Callback::from(move |_| {
                                let users = users.clone();

                                spawn_local(async move {

                                    let token: String = LocalStorage::get("token").unwrap_or_default();

                                    let url = format!("{}/users/{}", API_BASE, id);
                                    web_sys::console::log_1(&format!("Deleting: {}", url).into());

                                    let response = Request::delete(&url)
                                        .header("Authorization", &format!("Bearer {}", token))
                                        .send()
                                        .await;

                                    match response {
                                        Ok(resp) => {
                                            web_sys::console::log_1(&format!("Delete status: {}", resp.status()).into());

                                            if resp.status() == 200 {
                                                // ✅ Reload list after successful delete
                                                let refreshed = Request::get(&format!("{}/users", API_BASE))
                                                    .header("Authorization", &format!("Bearer {}", token))
                                                    .send()
                                                    .await
                                                    .unwrap()
                                                    .json::<Vec<User>>()
                                                    .await
                                                    .unwrap();

                                                users.set(refreshed);
                                            } else {
                                                web_sys::console::log_1(&"❌ Delete failed (401/403?)".into());
                                            }
                                        }
                                        Err(e) => {
                                            web_sys::console::log_1(&format!("Delete error: {:?}", e).into());
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
        </div>
    }
}

