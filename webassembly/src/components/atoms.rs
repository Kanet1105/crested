//! This module provides Bootstrap 5 style atomic components.
//! It includes the following components
//! 
//! - <Card> : a bordered-container for child components
//! - <Checkbox>
//! 
use yew::prelude::*;

/// The plain card component without a header.
#[derive(PartialEq, Properties)]
pub struct CardProps {
    pub include_header: bool,
    #[prop_or_default]
    pub name: &'static str,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Card(props: &CardProps) -> Html {
    if props.include_header {
        html! {
            <div class="card container">
                <div class="card-header">
                    <h3 class="card-title">{ props.name }</h3>
                </div>
                <div class="card-body">
                    { for props.children.iter() }
                </div>
            </div>
        }
    } else {
        html! {
            <div class="card container">
                <div class="card-body">
                    <h3 class="card-title">{ props.name }</h3>
                    { for props.children.iter() }
                </div>
            </div>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct CheckboxProps{
    pub name: &'static str,
}

#[function_component]
pub fn Checkbox(props: &CheckboxProps) -> Html {
    html! {
        <div class="form-check">
            <input class="form-check-input" type="checkbox" value="" id={ props.name } />
            <label class="form-check-label" for="flexCheckDefault">
                { props.name }
            </label>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct TestProps {
    pub name: &'static str,
}

#[function_component]
pub fn Test(props: &TestProps) -> Html {
    html! {
        <div class="input-group">
            <div class="input-group-text">
                <input class="form-check-input" type="checkbox" value="" aria-label="Checkbox for following text input" />
            </div>
            <input type="text" class="form-control" aria-label= { props.name } />
        </div>
    }
}