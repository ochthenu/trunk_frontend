use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PhotosProps {
    #[prop_or_default]
    pub category: Option<String>,
}

#[function_component(Photos)]
pub fn photos(props: &PhotosProps) -> Html {

    let all_images = vec![
        (
            "wash",
            vec![
                (
                    "/images/prep2.jpg",
                    "Fresh nozawana being carefully washed before preparation."
                ),
            ],
        ),

        (
            "prepare",
            vec![
                (
                    "/images/prep.jpg",
                    "Traditional preparation process using local ingredients and methods."
                ),
            ],
        ),

        (
            "pack",
            vec![
                (
                    "/images/prep3.jpg",
                    "Finished nozawana products being inspected and packaged for total customer satisfaction."
                ),
            ],
        ),

        (
            "all",
            vec![
                (
                    "/images/prep2.jpg",
                    "Fresh nozawana being carefully washed before preparation."
                ),

                (
                    "/images/prep.jpg",
                    "Traditional preparation process using local ingredients and methods."
                ),

                (
                    "/images/prep3.jpg",
                    "Finished nozawana products being inspected and packaged for total customer satisfaction."
                ),
            ],
        ),
    ];

    let category = props
        .category
        .clone()
        .unwrap_or_else(|| "all".to_string())
        .to_lowercase();

    let images = all_images
        .iter()
        .find(|(cat, _)| *cat == category)
        .map(|(_, imgs)| imgs.clone())
        .unwrap_or_default();

    html! {

        <main class="production-gallery">

            <h1>{ "Production Process" }</h1>

            <div class="photo-grid">

                {
                    for images.iter().map(|(src, desc)| html! {

                        <div class="photo-card">

                            <img
                                src={src.to_string()}
                                alt={desc.to_string()}
                            />

                            <p>{ *desc }</p>

                        </div>

                    })
                }

            </div>

            <p
                style="
                    text-align: center;
                    margin-top: 2rem;
                    font-style: italic;
                    color: #666;
                "
            >
                { "Photos courtesy of Takeuchi Nosan" }
            </p>

            {
                if category != "all" {

                    html! {

                        <div class="back-link">

                            <Link<Route> to={Route::Gallery}>
                                { "← Back to Gallery" }
                            </Link<Route>>

                        </div>
                    }

                } else {

                    html! {}

                }
            }

        </main>
    }
}