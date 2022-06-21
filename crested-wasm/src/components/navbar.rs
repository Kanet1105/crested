use yew::{html, Children, Context, Component, Html, Properties};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: &'static str,
    pub tabs: Children,
}

pub struct NavBar;

impl Component for NavBar {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                <div class="container">
                    // the brand button
                    <a class="navbar-brand" href="/">{"CRESTED"}</a>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNavAltMarkup" aria-controls="navbarNavAltMarkup" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
    
                    // the actual navigation menu
                    <div class="collapse navbar-collapse show" id="navbarNavAltMarkup">
                        <div class="navbar-nav">
                            <li class="nav-item">
                                <a class="nav-link" href="/App">{"Tab 1"}</a>
                            </li>
                            <li class="nav-item">
                                <a class="nav-link" href="/Videos">{"Tab 2"}</a>
                            </li>
                            <li class="nav-item">
                                <a class="nav-link" href="/Posts">{"Tab 3"}</a>
                            </li>
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
}