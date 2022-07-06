use std::rc::Rc;
use yew::prelude::*;

pub enum AppListActions {
    Next,
}

#[derive(PartialEq)]
pub struct AppList {
    list: Vec<String>,
    current_index: i32,
}

impl Default for AppList {
    fn default() -> Self {
        Self {
            list: vec![],
            current_index: 0,
        }
    }
}

impl Reducible for AppList {
    type Action = AppListActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_state = match action {
            AppListActions::Next => self.concatenate(),
        };

        Self {
            list: next_state,
            current_index: 0,
        }.into()
    }
}

impl AppList {
    fn get_last_index(&self) -> usize {
        self.list.len()
    }

    fn concatenate(&self) -> Vec<String> {
        let mut current = self.list.clone();
        let index = self.get_last_index();

        for i in index + 1..index + 5 {
            let name = i.to_string();
            current.push(name);
        }
        current
    }
}


#[derive(PartialEq, Properties)]
pub struct BoardProps {
    pub name: &'static str,
}

#[function_component]
pub fn Board(props: &BoardProps) -> Html {
    let app_list = use_reducer(AppList::default);

    let onclick = {
        let app_list = app_list.clone();
        Callback::from(move |_| app_list.dispatch(AppListActions::Next))
    };

    html! {
        <>
        <div class="card container mb-3">
            // the navigation tab header
            <h3>{ props.name }</h3>

            <div class="card-body">
            </div>
        </div>

        <div class="card container mb-3">
            { for app_list.list.iter().map(|app| html! {
                <Content name={ app.clone() } />
            }) }
        </div>

        <div class="container">
            <button {onclick} type="button" class="btn btn-outline-dark">{ "Next" }</button>
        </div>
        </>
    }
}

#[derive(PartialEq, Properties)]
pub struct ContentProps {
    pub name: String,
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
                        <h5 class="card-title">{ props.name.as_str() }</h5>
                        <p class="card-text">{ "This is a wider card with supporting text below as a natural lead-in to additional content." }</p>
                        <p class="card-text"><small class="text-muted">{ "Last updated 3 mins ago" }</small></p>
                    </div>
                </div>
            </div>
        </div>
    }
}