use crate::components::content::Content;

use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn CreateButton() -> Html {
    /*
    let (_, dispatch) = use_store::<State>();
    let onclick = dispatch.reduce_mut_callback(|s| {
        s.counter += 1;
        s.summary.push(s.counter.to_string());
    });
    */
    html! {
        <button>{" + 1 "}</button>
    }
}

#[derive(PartialEq, Properties)]
pub struct BoardProps {
    pub name: &'static str,
}

#[function_component]
pub fn Board(props: &BoardProps) -> Html {
    // let (state, _) = use_store::<State>();

    html! {
        <div class="card container">
            <h3>{"TRENDING"}</h3>
            <div class="card-header">
                <ul class="nav nav-tabs card-h  eader-tabs" role="tablist">
                    <li class="nav-item" role="presentation">
                        <a class="nav-link active" id="home-tab" data-bs-toggle="tab" href="#all" role="tab" aria-controls="all" aria-selected="true">{"All OS"}</a>
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
                        <Content />
                        <Content />
                        <Content />
                    </div>
                    <div class="tab-pane fade" id="windows" role="tabpanel" aria-labelledby="windows-tab">{""}</div>
                    <div class="tab-pane fade" id="mac" role="tabpanel" aria-labelledby="mac-tab">{""}</div>
                </div>
            </div>
        </div>
    }
}