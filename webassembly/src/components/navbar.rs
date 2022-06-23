//! Define a navigation on the header of the page.
use yew::prelude::*;

/// Define a tab in the navigation bar.
#[derive(PartialEq, Properties)]
pub struct NavTabProps {
    name: &'static str,
    hyperlink: &'static str,
}

#[function_component]
pub fn NavTab(props: &NavTabProps) -> Html {
    html! {
        <li class="nav-item">
            <a class="nav-link" href={props.hyperlink}>{props.name}</a>
        </li>
    }
}

/// Define a navigation bar
#[derive(PartialEq, Properties)]
pub struct NavBarProps {
    pub name: &'static str,
    pub children: Children,
}

#[function_component]
pub fn NavBar(props: &NavBarProps) -> Html {
    html! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
            <div class="container">
                <a class="navbar-brand" href="/">{ props.name }</a>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNavAltMarkup" aria-controls="navbarNavAltMarkup" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>

                // the actual navigation menu
                <div class="collapse navbar-collapse show" id="navbarNavAltMarkup">
                    <div class="navbar-nav me-auto">
                        { for props.children.iter() }
                    </div>

                    // login link
                    <form class="d-flex">
                        <a class="btn btn-outline-light" href="/login">{"Log in"}</a>
                    </form>
                </div>
            </div>
        </nav>
    }
}