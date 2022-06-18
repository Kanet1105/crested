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
            <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                <div class="container fw-bold">
                    // the brand button
                    <a class="navbar-brand" href="/">{"Crested"}</a>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNavAltMarkup" aria-controls="navbarNavAltMarkup" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>

                    // the actual navigation menu
                    <div class="collapse navbar-collapse show" id="navbarNavAltMarkup">
                        <div class="navbar-nav">
                            <a class="nav-link" href="/">{"Home"}</a>
                            <a class="nav-link" href="/">{"Features"}</a>
                            <a class="nav-link" href="/">{"Pricing"}</a>
                            <a class="nav-link" href="/">{"Disabled"}</a>
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
}
