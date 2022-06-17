mod components;

use yew::{Context, Component, Html, html};
use components::navbar::NavBar;

pub struct Index;

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <NavBar>
                </NavBar>
            </>
        }
    }
}