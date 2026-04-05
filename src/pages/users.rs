use yew::prelude::*;
use yew_router::prelude::*; // ✅ needed for navigation
use gloo::net::http::Request;
use gloo::storage::{LocalStorage, Storage};
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

    // ✅ navigator (SAFE, no unwrap)
    let navigator = use_navigator();

    // ✅ logout handler
    let on_logout = {
        let navigator = navigator.clone();

        Callback::from(move |_| {
            // remove token
            LocalStorage::delete("token");

            // redirect safely
            if let Some(nav) = navigator.clone() {
                nav.push(&crate::app::Route::Login);
            }
        })
    };

    // ✅ SAFE token retrieval
    let token = match LocalStorage::get::<String>("token") {
        Ok(t) => t,
        Err(_) => {
            return html! {
                <div>
                    <h1>{ "User Administration" }</h1>
                    <p>{ "Please log in to view users." }</p>
                </div>
            };
        }
    };

    let users = use_state(|| Vec::<User>::new());

    {
        let users = users.clone();
        let token = token.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {

                let url = format!("{}/users", API_BASE);

                match Request::get(&url)
                    .header("Authorization", &format!("Bearer {}", token))
                    .send()
                    .await
                {
                    Ok(resp) => {
                        match resp.json::<Vec<User>>().await {
                            Ok(data) => users.set(data),
                            Err(e) => web_sys::console::log_1(
                                &format!("JSON error: {:?}", e).into()
                            ),
                        }
                    }
                    Err(e) => web_sys::console::log_1(
                        &format!("Request error: {:?}", e).into()
                    ),
                }
            });

            || ()
        });
    }

    html! {
        <div>
            <h1>{ "User Administration" }</h1>

            // ✅ 👇 LOGOUT BUTTON HERE
            <button onclick={on_logout} style="margin-bottom: 10px;">
                { "Logout" }
            </button>

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

                                    // reload list
                                    match Request::get(&format!("{}/users", API_BASE))
                                        .header("Authorization", &format!("Bearer {}", token))
                                        .send()
                                        .await
                                    {
                                        Ok(resp) => {
                                            if let Ok(data) = resp.json::<Vec<User>>().await {
                                                users.set(data);
                                            }
                                        }
                                        Err(e) => web_sys::console::log_1(
                                            &format!("Reload error: {:?}", e).into()
                                        ),
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

