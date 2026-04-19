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
        LocalStorage::get("username").unwrap_or("Anonymous".to_string());

    let token: Option<String> = LocalStorage::get("token").ok();

    let is_admin = username == "nigel2";

    // ✅ LOAD POSTS
    {
        let posts = posts.clone();
        let token = token.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Some(token) = token {
                    let resp = Request::get(&format!("{}/posts", API_BASE))
                        .header("Authorization", &format!("Bearer {}", token))
                        .send()
                        .await;

                    if let Ok(resp) = resp {
                        if let Ok(data) = resp.json::<Vec<Post>>().await {
                            posts.set(data);
                        }
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
            if input.is_empty() {
                return;
            }

            // 🔐 block anonymous users
            if token.is_none() {
                web_sys::window()
                    .unwrap()
                    .alert_with_message("Please log in to post")
                    .unwrap();
                return;
            }

            let content = (*input).clone();
            let posts = posts.clone();
            let input = input.clone();
            let token = token.clone().unwrap();

            spawn_local(async move {
                let resp = Request::post(&format!("{}/posts", API_BASE))
                    .header("Content-Type", "application/json")
                    .header("Authorization", &format!("Bearer {}", token))
                    .json(&CreatePost { content })
                    .unwrap()
                    .send()
                    .await;

                if let Ok(resp) = resp {
                    if resp.status() != 200 {
                        web_sys::console::log_1(
                            &format!("POST failed: {}", resp.status()).into(),
                        );
                    }
                }

                // 🔄 reload posts
                if let Ok(resp) = Request::get(&format!("{}/posts", API_BASE))
                    .header("Authorization", &format!("Bearer {}", token))
                    .send()
                    .await
                {
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
            let posts = posts.clone();
            let token = token.clone();

            spawn_local(async move {
                if let Some(token) = token.clone() {
                    let _ = Request::delete(&format!("{}/posts/{}", API_BASE, id))
                        .header("Authorization", &format!("Bearer {}", token))
                        .send()
                        .await;

                    // 🔄 reload
                    if let Ok(resp) = Request::get(&format!("{}/posts", API_BASE))
                        .header("Authorization", &format!("Bearer {}", token))
                        .send()
                        .await
                    {
                        if let Ok(data) = resp.json::<Vec<Post>>().await {
                            posts.set(data);
                        }
                    }
                }
            });
        })
    };

    // 🧹 CLEAR ALL (ADMIN)
    let on_clear_all = {
        let posts = posts.clone();
        let token = token.clone();
        let is_admin = is_admin;

        Callback::from(move |_| {
            if !is_admin {
                return;
            }

            if web_sys::window()
                .unwrap()
                .confirm_with_message("Clear ALL posts?")
                .unwrap()
            {
                let posts = posts.clone();
                let token = token.clone();

                spawn_local(async move {
                    if let Some(token) = token {
                        for post in (*posts).clone() {
                            let _ = Request::delete(&format!("{}/posts/{}", API_BASE, post.id))
                                .header("Authorization", &format!("Bearer {}", token))
                                .send()
                                .await;
                        }

                        posts.set(Vec::new());
                    }
                });
            }
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
                <p>{ format!("Logged in as: {}", username) }</p>

                <button onclick={on_logout}>
                    { "Logout" }
                </button>
            </div>

            <h1>{ "Blog" }</h1>

            <div class="blog-form">
                <h3>{ "Write a post:" }</h3>

                <textarea
                    value={(*input).clone()}
                    oninput={on_input}
                    placeholder="Write something..."
                />

                <div class="button-row">
                    <button onclick={on_add} disabled={token.is_none()}>
                        { "Add Post" }
                    </button>

                    {
                        if is_admin {
                            html! {
                                <button onclick={on_clear_all}>
                                    { "Clear ALL Posts" }
                                </button>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>

                {
                    if token.is_none() {
                        html! {
                            <p style="color: #888;">
                                { "You must be logged in to post." }
                            </p>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>

            <div class="blog-posts">
                <h3>{ "Posts:" }</h3>

                {
                    if posts.is_empty() {
                        html! { <p>{ "No posts yet." }</p> }
                    } else {
                        html! {
                            for posts.iter().map(|post| {
                                let can_delete =
                                    is_admin || post.username == username;

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