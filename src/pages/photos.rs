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
    // Images grouped by category
    let all_images = vec![
        ("nature", vec!["/images/nature2.jpg"]),
        ("cities", vec!["/images/city2.jpg"]),
        ("people", vec!["/images/people2.jpg"]),
        (
            "all",
            vec!["/images/cat.jpg", "/images/dog.jpg", "/images/sunset.png"],
        ),
    ];

    // Determine category, default to "all"
    let category = props
        .category
        .clone()
        .unwrap_or_else(|| "all".to_string())
        .to_lowercase();

    let images: Vec<&str> = all_images
        .iter()
        .find(|(cat, _)| *cat == category)
        .map(|(_, imgs)| imgs.clone())
        .unwrap_or_default();

    html! {
        <main class="photo-gallery">
            { for images.iter().map(|src| html! {
                <img src={src.to_string()} alt={src.to_string()} />
            }) }

            { if category != "all" {
                html! {
                    <div style="margin-top: 2rem;">
                        <Link<Route> to={Route::Gallery}>{ "← Back to Gallery" }</Link<Route>>
                    </div>
                }
            } else {
                html! {}
            }}
        </main>
    }
}
