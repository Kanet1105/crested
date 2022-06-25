use std::convert::From;

use yew::prelude::*;
use yew::html::ChildrenRenderer;
use yew::virtual_dom::VChild;

#[derive(PartialEq, Properties)]
pub struct BoardProps {
    pub name: &'static str,
    pub include_nav: bool,
}

#[function_component]
pub fn Board(props: &BoardProps) -> Html {
    if props.include_nav {
        html! {
            <div class="card container">

                // the navigation tab header
                <h3>{ props.name }</h3>
                <div class="card-header">
                    <ul class="nav nav-tabs card-h  eader-tabs" role="tablist">
                        <li class="nav-item" role="presentation">
                            <a class="nav-link" id="home-tab" data-bs-toggle="tab" href="#all" role="tab" aria-controls="all" aria-selected="true">{"All OS"}</a>
                        </li>
                        <li class="nav-item" role="presentation">
                            <a class="nav-link" id="profile-tab" data-bs-toggle="tab" href="#windows" role="tab" aria-controls="windows" aria-selected="false">{"Windows"}</a>
                        </li>
                        <li class="nav-item" role="presentation">
                            <a class="nav-link" id="profile-tab" data-bs-toggle="tab" href="#mac" role="tab" aria-controls="mac" aria-selected="false">{"Mac"}</a>
                        </li>
                    </ul>
                </div>

                <div class="card-body">
                    <div class="tab-content">
                        <div class="tab-pane fade show active" id="all" role="tabpanel" aria-labelledby="all-tab">
                        </div>
                        <div class="tab-pane fade" id="windows" role="tabpanel" aria-labelledby="windows-tab">{""}</div>
                        <div class="tab-pane fade" id="mac" role="tabpanel" aria-labelledby="mac-tab">{""}</div>
                    </div>
                </div>

            </div>
        }
    } else {
        html! {
            <div class="card container">

                // the navigation tab header
                <h3>{ props.name }</h3>

                <div class="card-body">

                </div>
            </div>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct BoardTabProps {
    pub name: &'static str,
}

#[function_component]
pub fn BoardTab(props: &BoardTabProps) -> Html {
    let mut do_nothing: String = String::from("#"); // adding a "#" before the href does not route.
    do_nothing.push_str(props.name);

    html! {
        <li class="nav-item" role="presentation">
            <a class="nav-link" data-bs-toggle="tab" href={do_nothing} role="tab" aria-controls={props.name} aria-selected="true">{props.name}</a>
        </li>
    }
}

#[derive(PartialEq, Properties)]
pub struct ContentProps {
    pub name: &'static str,
}

#[function_component]
pub fn Content (props: &ContentProps) -> Html {
    html! {
        <div class="card container mb-3">
            <div class="row g-0">
                <div class="col-5 col-sm-4">
                    // <img src="assets/images/bs-images/img-3x4.png" class="img-fluid w-100" alt="card-horizontal-image">
                </div>
                <div class="col-7 col-sm-8">
                    <div class="card-body">
                        <h5 class="card-title">{"Card title"}</h5>
                        <p class="card-text">{"This is a wider card with supporting text below as a natural lead-in to additional content."}</p>
                        <p class="card-text"><small class="text-muted">{"Last updated 3 mins ago"}</small></p>
                    </div>
                </div>
            </div>
        </div>
    }
}