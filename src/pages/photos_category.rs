use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub category: String,
}

#[function_component(PhotosCategory)]
pub fn photos_category(props: &Props) -> Html {

    let items = match props.category.as_str() {

        "lightly-pickled" => vec![
            (
                "/images/light1.jpg",
                "A light-flavored dish that brings out the deliciousness of Nozawana."
            ),
            (
                "/images/light2.jpg",
                "Freshly prepared lightly pickled leaves."
            ),
            (
                "/images/light3.jpg",
                "Served alongside warm rice."
            ),
        ],

        "wasabi-flavored" => vec![
            (
                "/images/wasabi1.jpg",
                "Wasabi flavored nozawana with extra spice."
            ),
            (
                "/images/wasabi2.jpg",
                "Bright green leaves with wasabi seasoning."
            ),
        ],

        "wasabi-soy" => vec![
            (
                "/images/soy1.jpg",
                "Wasabi and soy flavored nozawana."
            ),
            (
                "/images/soy2.jpg",
                "Savory soy aroma with mild heat."
            ),
        ],

        _ => vec![],
    };

    html! {

        <main class="photos-page">

            <div class="back-link">
                <Link<Route> to={Route::Gallery}>
                    { "← Back to Gallery" }
                </Link<Route>>
            </div>

            <h1>
                { format!("{} Gallery", props.category) }
            </h1>

            <div class="photo-grid">

                {
                    items.iter().map(|(image, desc)| {

                        html! {

                            <div class="photo-card">

                                <img src={(*image).to_string()} />

                                <p>{ *desc }</p>

                            </div>
                        }

                    }).collect::<Html>()
                }

            </div>

        </main>
    }
}