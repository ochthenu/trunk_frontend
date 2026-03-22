use yew::prelude::*;
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;

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
        use_effect_with((), move |_| {
            spawn_local(async move {
                let response = Request::get("/users")
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<User>>()
                    .await
                    .unwrap();

                users.set(response);
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
                                        &format!("/users/{}", id)
                                    )
                                    .send()
                                    .await;

                                    // reload list
                                    let response = Request::get("/users")
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

