use yew::{Context, Component, Html, html};
use crate::components::navbar::NavBar;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <header><NavBar nav_title="CRESTED App" /></header>
            </>
        }
    }
}