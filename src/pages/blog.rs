use gloo::net::http::Request;
use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::{File, FormData, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_router::prelude::*;

#[cfg(debug_assertions)]
const API_BASE: &str = "/api";

#[cfg(not(debug_assertions))]
const API_BASE: &str = "https://rustbackend-production.up.railway.app";

#[derive(Deserialize, Clone, PartialEq)]
struct Post {
    id: i32,
    username: String,
    content: String,
    image_url: Option<String>,
}

#[derive(Serialize)]
struct CreatePost {
    content: String,
    image_url: Option<String>,
}

#[function_component(Blog)]
pub fn blog() -> Html {
    let navigator = use_navigator();

    let posts = use_state(|| Vec::<Post>::new());
    let input = use_state(|| "".to_string());
    let selected_file = use_state(|| None::<File>);

    let username: String = LocalStorage::get("username").unwrap_or("".to_string());

    let token: Option<String> = LocalStorage::get("token").ok();
    let is_logged_in = token.is_some();

    let is_admin = username.trim().to_lowercase() == "nigel2";

    // ✅ LOAD POSTS
    {
        let posts = posts.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(resp) = Request::get(&format!("{}/posts", API_BASE)).send().await {
                    if let Ok(data) = resp.json::<Vec<Post>>().await {
                        posts.set(data);
                    }
                }
            });
            || ()
        });
    }

    // ✍️ typing
    let on_input = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: HtmlTextAreaElement = e.target_unchecked_into();
            input.set(textarea.value());
        })
    };

    // 📷 file selection
    let on_file_change = {
        let selected_file = selected_file.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();

            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    selected_file.set(Some(file));
                }
            }
        })
    };

    // ➕ ADD POST
    let on_add = {
        let posts = posts.clone();
        let input = input.clone();
        let token = token.clone();
        let selected_file = selected_file.clone();

        Callback::from(move |_| {
            if input.is_empty() || token.is_none() {
                return;
            }

            let content = (*input).clone();
            let posts = posts.clone();
            let input = input.clone();
            let token = token.clone().unwrap();
            let selected_file = (*selected_file).clone();

            spawn_local(async move {
                let mut image_url: Option<String> = None;
                if let Some(file) = selected_file {
                    let form = FormData::new().unwrap();

                    form.append_with_blob_and_filename("file", &file, &file.name())
                        .unwrap();

                    let upload = Request::post(&format!("{}/upload", API_BASE))
                        .body(form)
                        .send()
                        .await;

                    match upload {
                        Ok(resp) => {
                            let text = resp.text().await.unwrap_or_default();

                            web_sys::console::log_1(&format!("Upload returned: {}", text).into());

                            image_url = Some(text);
                        }
                        Err(err) => {
                            web_sys::console::log_1(&format!("Upload failed: {:?}", err).into());
                        }
                    }
                }

                let resp = Request::post(&format!("{}/posts", API_BASE))
                    .header("Content-Type", "application/json")
                    .header("Authorization", &format!("Bearer {}", token))
                    .json(&CreatePost { content, image_url })
                    .unwrap()
                    .send()
                    .await;

                match resp {
                    Ok(r) => {
                        web_sys::console::log_1(
                            &format!("POST /posts returned {}", r.status()).into(),
                        );
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("POST failed: {:?}", e).into());
                    }
                }

                if let Ok(resp) = Request::get(&format!("{}/posts", API_BASE)).send().await {
                    if let Ok(data) = resp.json::<Vec<Post>>().await {
                        posts.set(data);
                    }
                }

                input.set(String::new());
            });
        })
    };

    // ❌ DELETE POST
    let on_delete = {
        let posts = posts.clone();
        let token = token.clone();

        Callback::from(move |id: i32| {
            if token.is_none() {
                return;
            }

            let posts = posts.clone();
            let token = token.clone().unwrap();

            spawn_local(async move {
                let _ = Request::delete(&format!("{}/posts/{}", API_BASE, id))
                    .header("Authorization", &format!("Bearer {}", token))
                    .send()
                    .await;

                if let Ok(resp) = Request::get(&format!("{}/posts", API_BASE)).send().await {
                    if let Ok(data) = resp.json::<Vec<Post>>().await {
                        posts.set(data);
                    }
                }
            });
        })
    };

    // 🔐 logout
    let on_logout = {
        let navigator = navigator.clone();

        Callback::from(move |_| {
            LocalStorage::delete("token");
            LocalStorage::delete("username");

            if let Some(nav) = navigator.clone() {
                nav.push(&crate::app::Route::Login);
            }

            web_sys::window().unwrap().location().reload().unwrap();
        })
    };

    html! {
                <div class="page-content">

                    <div class="blog-header">
                        {
                            if is_logged_in {
                                html! { <p>{ format!("Logged in as: {}", username) }</p> }
                            } else {
                                html! { <p>{ "Not logged in" }</p> }
                            }
                        }

                        {
                            if is_logged_in {
                                html! {
                                    <button onclick={on_logout}>
                                        { "Logout" }
                                    </button>
                                }
                            } else {
                                html! {
                                    <button onclick={
                                        let navigator = navigator.clone();
                                        Callback::from(move |_| {
                                            if let Some(nav) = navigator.clone() {
                                                nav.push(&crate::app::Route::Login);
                                            }
                                        })
                                    }>
                                        { "Login" }
                                    </button>
                                }
                            }
                        }
                    </div>

                    <h1>{ "Blog" }</h1>

                    // ✅ FORM AREA
                    <div class="blog-form">
            <h3>{ "Write a post:" }</h3>

                    {
                        if is_logged_in {
                            html! {
                                <>
                                    <textarea
                                        value={(*input).clone()}
                                        oninput={on_input}
                                        placeholder="Write something..."
                                    />

                                    <input
            type="file"
            accept="image/*"
            onchange={on_file_change}
        />

                                    <div class="button-row">
                                        <button onclick={on_add}>
                                            { "Add Post" }
                                        </button>
                                    </div>
                                </>
                            }
                        } else {
                            html! {
                        <>
                            <p>
                                { "Register an account, then log in to create and manage your own posts." }
                            </p>


                            <div class="button-row">
                                <button onclick={
                                    let navigator = navigator.clone();
                                    Callback::from(move |_| {
                                        if let Some(nav) = navigator.clone() {
                                            nav.push(&crate::app::Route::Register);
                                        }
                                    })
                                }>
                                    { "Register" }
                                </button>

                                <button onclick={
                                    let navigator = navigator.clone();
                                    Callback::from(move |_| {
                                        if let Some(nav) = navigator.clone() {
                                            nav.push(&crate::app::Route::Login);
                                        }
                                    })
                                }>
                                    { "Login" }
                                </button>
                            </div>
                        </>
                    }
                }
            }
            </div>

                    // ✅ POSTS
                    <div class="blog-posts">
                        <h3>{ "Posts:" }</h3>

                        {
                            if posts.is_empty() {
                                html! { <p>{ "No posts yet." }</p> }
                            } else {
                                html! {
                                    for posts.iter().map(|post| {
                                        let can_delete =
                                            is_logged_in &&
                                            (post.username == username || is_admin);

                                        let id = post.id;

                                        let on_delete = {
                                            let on_delete = on_delete.clone();
                                            Callback::from(move |_| on_delete.emit(id))
                                        };

                                        html! {
        <div class="post-item">
            <strong>{ format!("{}: ", post.username) }</strong>

            {
                if let Some(url) = &post.image_url {
                    html! {
                        <div>
                            <img
                                src={format!("{}{}", API_BASE, url)}
                                class="post-image"
                            />
                        </div>
                    }
                } else {
                    html! {}
                }
            }

            <div>{ &post.content }</div>

            {
                if can_delete {
                    html! {
                        <button onclick={on_delete}>
                            { "Delete" }
                        </button>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
                                    })
                                }
                            }
                        }
                    </div>
                </div>
            }
}
