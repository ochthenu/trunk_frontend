use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Gallery)]
pub fn gallery() -> Html {

    html! {

        <main class="gallery">

            <h1>{ "Gallery Categories" }</h1>

            <ul>

                <li>
                    <Link<Route>
                        to={Route::PhotosCategory {
                            category: "lightly-pickled".into()
                        }}
                    >
                        { "Lightly Pickled" }
                    </Link<Route>>
                </li>

                <li>
                    <Link<Route>
                        to={Route::PhotosCategory {
                            category: "wasabi-flavored".into()
                        }}
                    >
                        { "Wasabi Flavored" }
                    </Link<Route>>
                </li>

                <li>
                    <Link<Route>
                        to={Route::PhotosCategory {
                            category: "wasabi-soy".into()
                        }}
                    >
                        { "Wasabi and Soy Flavored" }
                    </Link<Route>>
                </li>

            </ul>

        </main>
    }
}
