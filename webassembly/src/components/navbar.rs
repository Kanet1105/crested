use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NavBarProps {
    pub name: &'static str,
    #[prop_or_default]
    pub children: ChildrenWithProps<NavBarTab>,
}

#[function_component]
pub fn NavBar(props: &NavBarProps) -> Html {
    html! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
            <div class="container">

                // the navbar name
                <a class="navbar-brand" href="/">{ props.name }</a>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNavAltMarkup" aria-controls="navbarNavAltMarkup" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>

                // the div where items are.
                <div class="collapse navbar-collapse show" id="navbarNavAltMarkup">

                    // nav-items go into here.
                    <div class="navbar-nav me-auto">
                        { for props.children.iter() }
                    </div>

                    // the login button
                    <form class="d-flex">
                        <a class="btn btn-outline-light" href="/login">{"Log in"}</a>
                    </form>

                </div>
                
            </div>
        </nav>
    }
}

#[derive(PartialEq, Properties)]
pub struct NavTabProps {
    pub name: &'static str,
    pub hlink: &'static str,
}

#[function_component]
pub fn NavBarTab(props: &NavTabProps) -> Html {
    html! {
        <li class="nav-item">
            <a class="nav-link" href={props.hlink}>{props.name}</a>
        </li>
    }
}