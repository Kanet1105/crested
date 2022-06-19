use yew::{Context, Component, Html, html};
use crate::components::navbar::NavBar;

pub struct Features;

impl Component for Features {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            </>
        }
    }
}