pub mod components;
pub mod pages;

use pages::developer::Developer;
use pages::home::Home;
use pages::introduction::GettingStarted;
use pages::notfound::NotFound;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/getting-started")]
    GettingStarted,
    #[at("/developer")]
    Developer,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home />},
        Route::GettingStarted => html! {<GettingStarted />},
        Route::Developer => html! {<Developer />},
        Route::NotFound => html! {<NotFound />},
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}