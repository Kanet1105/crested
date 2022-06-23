use crate::components::navbar::{NavBar, NavTab};

use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    html! {
        <NavBar name="CRESTED" tabs={} />
    }
}