use yew::prelude::*;
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;

// ✅ ALWAYS use production backend (simple + reliable)
const API_BASE: &str = "https://rustbackend-production.up.railway.app";

#[derive(Clone, PartialEq, serde::Deserialize)]
struct User {
    id: i32,
    name: String,
}

#[function_component(Users)]
pub fn users() -> Html {
    let users = use_state(|| Vec::<User>::new());

    // Load users
    {
        let users = users.clone();

        use_effect(move || {
            spawn_local(async move {
                match Request::get(&format!("{}/users", API_BASE))
                    .send()
                    .await
                {
                    Ok(resp) => {
                        if let Ok(data) = resp.json::<Vec<User>>().await {
                            users.set(data);
                        } else {
                            web_sys::console::log_1(&"Failed to parse users".into());
                        }
                    }
                    Err(_) => {
                        web_sys::console::log_1(&"Failed to fetch users".into());
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
                                    if let Ok(resp) = Request::get(&format!("{}/users", API_BASE))
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
        </div>
    }
}

