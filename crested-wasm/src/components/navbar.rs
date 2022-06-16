use yew::{Context, Component, Html, html};

pub struct NavBar;

impl Component for NavBar {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav id="global-navbar">
                <div>
                    <button><a>{ "menu 1" }</a></button>
                </div>
                    <button><a>{ "menu 2" }</a></button>
                <div>

                </div>
            </nav>
        }
    }
}