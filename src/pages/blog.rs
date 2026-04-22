use yew::prelude::*;
use yew_router::prelude::*;
use gloo::net::http::Request;
use gloo::storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlTextAreaElement;
use serde::{Deserialize, Serialize};

#[cfg(debug_assertions)]
const API_BASE: &str = "/api";

#[cfg(not(debug_assertions))]
const API_BASE: &str = "https://rustbackend-production.up.railway.app";

#[derive(Deserialize, Clone, PartialEq)]
struct Post {
    id: i32,
    username: String,
    content: String,
}

#[derive(Serialize)]
struct CreatePost {
    content: String,
}

#[function_component(Blog)]
pub fn blog() -> Html {
    let navigator = use_navigator();

    let posts = use_state(|| Vec::<Post>::new());
    let input = use_state(|| "".to_string());

    let username: String =
        LocalStorage::get("username").unwrap_or("".to_string());

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

    // ➕ ADD POST
    let on_add = {
        let posts = posts.clone();
        let input = input.clone();
        let token = token.clone();

        Callback::from(move |_| {
            if input.is_empty() || token.is_none() {
                return;
            }

            let content = (*input).clone();
            let posts = posts.clone();
            let input = input.clone();
            let token = token.clone().unwrap();

            spawn_local(async move {
                let _ = Request::post(&format!("{}/posts", API_BASE))
                    .header("Content-Type", "application/json")
                    .header("Authorization", &format!("Bearer {}", token))
                    .json(&CreatePost { content })
                    .unwrap()
                    .send()
                    .await;

                // reload posts
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

            web_sys::window()
                .unwrap()
                .location()
                .reload()
                .unwrap();
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

                                <div class="button-row">
                                    <button onclick={on_add}>
                                        { "Add Post" }
                                    </button>
                                </div>
                            </>
                        }
                    } else {
                        html! {
                            <p>{ "Log in to post" }</p>
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
                                        { &post.content }

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