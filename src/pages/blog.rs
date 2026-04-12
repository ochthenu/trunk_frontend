use yew::prelude::*;
use gloo::storage::{LocalStorage, Storage};

#[function_component(Blog)]
pub fn blog() -> Html {
    let post_input = use_state(|| "".to_string());
    let posts = use_state(|| Vec::<String>::new());
    let is_logged_in = LocalStorage::get::<String>("token").is_ok();

    let on_input = {
        let post_input = post_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            post_input.set(input.value());
        })
    };

    let on_add_post = {
        let post_input = post_input.clone();
        let posts = posts.clone();

        Callback::from(move |_| {
            if post_input.is_empty() {
                return;
            }

            let mut new_posts = (*posts).clone();
            new_posts.push((*post_input).clone());
            posts.set(new_posts);

            post_input.set(String::new());
        })
    };

    html! {
        <div class="page-content">
            <h1>{ "Blog" }</h1>

            if is_logged_in {
                <div class="blog-form">
                    <h3>{ "Create a post" }</h3>

                    <textarea
                        placeholder="Write something..."
                        value={(*post_input).clone()}
                        oninput={on_input}
                    />

                    <button onclick={on_add_post}>
                        { "Post" }
                    </button>
                </div>
            } else {
                <p>{ "Please log in to create posts." }</p>
            }

            <div class="blog-posts">
                <h3>{ "Posts" }</h3>

                {
                    if posts.is_empty() {
                        html! { <p>{ "No posts yet." }</p> }
                    } else {
                        html! {
                            <ul>
                                {
                                    for posts.iter().map(|post| html! {
                                        <li class="post-item">{ post }</li>
                                    })
                                }
                            </ul>
                        }
                    }
                }
            </div>
        </div>
    }
}
