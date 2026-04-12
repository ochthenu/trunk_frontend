use yew::prelude::*;
use yew_router::prelude::*;
use gloo::storage::{LocalStorage, Storage};

use crate::pages::{
    About, Blog, Contact, Faq, Gallery, Home, Photos, Users, Register, Login,
};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[at("/photos")]
    PhotosAll,
    #[at("/photos/:category")]
    PhotosCategory { category: String },
    #[at("/gallery")]
    Gallery,
    #[at("/blog")]
    Blog,
    #[at("/faq")]
    Faq,
    #[at("/users")]
    Users,
    #[at("/register")]
    Register,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Contact => html! { <Contact /> },
        Route::PhotosAll => html! { <Photos /> },
        Route::PhotosCategory { category } => {
            html! { <Photos category={Some(category)} /> }
        }
        Route::Gallery => html! { <Gallery /> },
        Route::Blog => html! { <Blog /> },
        Route::Faq => html! { <Faq /> },
        Route::Users => html! { <Users /> },
        Route::Register => html! { <Register /> },
        Route::Login => html! { <Login /> },
        Route::NotFound => html! { <h1>{ "404 – Page not found" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    // ✅ Check token
    let token: Option<String> = LocalStorage::get("token").ok();
    let is_logged_in = token.is_some();

    html! {
        <BrowserRouter>
            <main class="app-container">
                <nav class="main-nav">
                    <ul>
                        <li><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></li>
                        <li><Link<Route> to={Route::About}>{ "About" }</Link<Route>></li>
                        <li><Link<Route> to={Route::Contact}>{ "Contact" }</Link<Route>></li>
                        <li><Link<Route> to={Route::PhotosAll}>{ "Photos" }</Link<Route>></li>
                        <li><Link<Route> to={Route::Gallery}>{ "Gallery" }</Link<Route>></li>
                        <li><Link<Route> to={Route::Blog}>{ "Blog" }</Link<Route>></li>
                        <li><Link<Route> to={Route::Faq}>{ "FAQ" }</Link<Route>></li>

                        {
                            if is_logged_in {
                                html! {
                                    <>
                                        <li><Link<Route> to={Route::Users}>{ "Users" }</Link<Route>></li>
                                    </>
                                }
                            } else {
                                html! {
                                    <>
                                        <li><Link<Route> to={Route::Register}>{ "Register" }</Link<Route>></li>
                                        <li><Link<Route> to={Route::Login}>{ "Login" }</Link<Route>></li>
                                    </>
                                }
                            }
                        }
                    </ul>
                </nav>

                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}
