use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AppState {
    pub refresh_counter: u32,
}

impl Default for AppState {
    fn default() -> Self {
        Self { refresh_counter: 0 }
    }
}

pub type AppContext = UseReducerHandle<AppState>;

#[derive(Clone)]
pub enum AppAction {
    Refresh,
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AppAction::Refresh => AppState { refresh_counter: self.refresh_counter + 1 }.into(),
        }
    }
}
