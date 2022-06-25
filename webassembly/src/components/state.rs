use std::rc::Rc;
use yew::prelude::*;

pub enum StateAction {
    Add,
}

pub struct State {
    pub contents: Vec<Html>,
}

impl Default for State {
    fn default() -> Self {
        Self { 
            contents: Vec::new(),
        }
    }
}

impl Reducible for State {
    type Action = StateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let modified_contents = match action {
            StateAction::Add => self.contents.clone().push(html! { "the new content" }),
        };

        Self { contents: modified_contents }.into()
    }
}

pub type StateContext = UseReducerHandle<State>;
