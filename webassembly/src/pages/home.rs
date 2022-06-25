use crate::components::atoms::{Card, Checkbox, Test};
use crate::components::board::{Board};
use crate::components::navbar::{NavBar, NavBarTab};
use crate::components::state::{StateContext};

use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    html! {
        <>
            <header class="mb-3">
                <NavBar name="CRESTED">
                    <NavBarTab name="Home" hlink="/" />
                    <NavBarTab name="Getting Started" hlink="/getting-started" />
                    <NavBarTab name="Developer" hlink="/developer" />
                </NavBar>
            </header>

            <div class="mb-3">
                <Card include_header={true} name="Filter">
                    <Card include_header={false} name="OS">
                        <Test name="Windows" />
                    </Card>
                </Card>
            </div>
        </>
    }
}