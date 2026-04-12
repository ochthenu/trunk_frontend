use yew::prelude::*;
use yew_router::prelude::*;
use gloo::storage::{LocalStorage, Storage};
use web_sys::HtmlTextAreaElement;

#[function_component(Blog)]
pub fn blog() -> Html {
    let navigator = use_navigator();

    // ✅ Load posts
    let posts = use_state(|| {
        LocalStorage::get("posts").unwrap_or(Vec::<(String, String)>::new())
    });

    let input = use_state(|| "".to_string());

    let username: String =
        LocalStorage::get("username").unwrap_or("Anonymous".to_string());

    let is_admin = username == "nigel2";

    // ✍️ typing
    let on_input = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: HtmlTextAreaElement = e.target_unchecked_into();
            input.set(textarea.value());
        })
    };

    // ➕ add post
    let on_add = {
        let posts = posts.clone();
        let input = input.clone();
        let username = username.clone();

        Callback::from(move |_| {
            if input.is_empty() {
                return;
            }

            let mut new_posts = (*posts).clone();
            new_posts.push((username.clone(), (*input).clone()));

            posts.set(new_posts.clone());
            let _ = LocalStorage::set("posts", new_posts);

            input.set(String::new());
        })
    };

    // ❌ delete one
    let on_delete = {
        let posts = posts.clone();
        let username = username.clone();
        let is_admin = is_admin;

        Callback::from(move |index: usize| {
            let mut new_posts = (*posts).clone();

            if let Some((post_user, _)) = new_posts.get(index) {
                if *post_user == username || is_admin {
                    new_posts.remove(index);
                }
            }

            posts.set(new_posts.clone());
            let _ = LocalStorage::set("posts", new_posts);
        })
    };

    // 🧹 clear all (admin)
    let on_clear_all = {
        let posts = posts.clone();
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
                posts.set(Vec::new());
                LocalStorage::delete("posts");
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
                    <button onclick={on_add}>
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
            </div>

            <div class="blog-posts">
                <h3>{ "Posts:" }</h3>

                {
                    if posts.is_empty() {
                        html! { <p>{ "No posts yet." }</p> }
                    } else {
                        html! {
                            for posts.iter().enumerate().map(|(i, (user, post))| {
                                let on_delete = {
                                    let on_delete = on_delete.clone();
                                    Callback::from(move |_| on_delete.emit(i))
                                };

                                html! {
                                    <div class="post-item">
                                        <strong>{ format!("{}: ", user) }</strong>
                                        { post }

                                        {
                                            if *user == username || is_admin {
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