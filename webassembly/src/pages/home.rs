use crate::components::atoms::{Card, Checkbox, Test};
use crate::components::board::{Board};
use crate::components::navbar::{NavBar, NavBarTab};

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
            <Board name="Applications" />
        </div>
        </>
    }
}