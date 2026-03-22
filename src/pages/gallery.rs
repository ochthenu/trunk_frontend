use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Gallery)]
pub fn gallery() -> Html {
    html! {
        <main class="gallery">
            <h1>{ "Gallery Categories" }</h1>
            <ul>
                <li><Link<Route> to={Route::PhotosCategory { category: "nature".into() }}>{ "Nature" }</Link<Route>></li>
                <li><Link<Route> to={Route::PhotosCategory { category: "cities".into() }}>{ "Cities" }</Link<Route>></li>
                <li><Link<Route> to={Route::PhotosCategory { category: "people".into() }}>{ "People" }</Link<Route>></li>
            </ul>
        </main>
    }
}
