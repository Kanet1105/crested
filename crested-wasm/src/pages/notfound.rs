use yew::prelude::*;

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div class="container">
            <h2>{"404 : Page Not Found.."}</h2>
        </div>
    }
}