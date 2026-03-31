use yew::prelude::*;
use gloo::net::http::Request;
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

    // ✅ Correct hook (runs once reliably)
    {
        let users = users.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {

                let url = format!("{}/users", API_BASE);
                web_sys::console::log_1(&format!("Fetching: {}", url).into());

                match Request::get(&url).send().await {
                    Ok(resp) => {
                        web_sys::console::log_1(&format!("Status: {}", resp.status()).into());

                        match resp.json::<Vec<User>>().await {
                            Ok(data) => {
                                web_sys::console::log_1(&format!("Users: {:?}", data).into());
                                users.set(data);
                            }
                            Err(e) => {
                                web_sys::console::log_1(&format!("JSON error: {:?}", e).into());
                            }
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

                                    let _ = Request::delete(
                                        &format!("{}/users/{}", API_BASE, id)
                                    )
                                    .send()
                                    .await;

                                    // reload list
                                    let response = Request::get(&format!("{}/users", API_BASE))
                                        .send()
                                        .await
                                        .unwrap()
                                        .json::<Vec<User>>()
                                        .await
                                        .unwrap();

                                    users.set(response);
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

