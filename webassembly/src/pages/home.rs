use crate::components::navbar::{NavBar, NavBarTab};
use crate::components::board::{Board};

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

        <div>
            <Board name="TRENDING" include_nav={true}>
                
            </Board>
        </div>
        </>
    }
}