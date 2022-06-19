mod components;
mod pages;

use yew::prelude::*;
use yew_router::prelude::*;
use pages::{home, features};

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/features")]
    Features,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <home::Home /> },
        Route::Features => html! { <features::Features /> },
    }
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <pages::home::Home />
            </>
        }
    }
}
