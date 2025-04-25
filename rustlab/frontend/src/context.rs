//! # Application Context Module
//!
//! This module defines the application state management and context for the spreadsheet.
//! It provides a central state mechanism using Yew's reducer pattern for components
//! to interact with shared application data.

use std::rc::Rc;
use yew::prelude::*;

/// Represents the global application state.
///
/// This structure maintains state that needs to be shared across components,
/// such as the refresh counter used to trigger UI updates.
#[derive(Clone, PartialEq)]
pub struct AppState {
    /// Counter that increments each time the UI needs to be refreshed.
    /// Components can watch this value to detect when they should update.
    pub refresh_counter: u32,
}

impl Default for AppState {
    /// Creates the default application state.
    ///
    /// # Returns
    ///
    /// A new `AppState` with initial values
    fn default() -> Self {
        Self { refresh_counter: 0 }
    }
}

/// Type alias for a handle to the application context.
///
/// This provides components with access to the application state and actions.
pub type AppContext = UseReducerHandle<AppState>;

/// Actions that can be performed on the application state.
///
/// This enum represents the various ways components can update the shared state.
#[derive(Clone)]
pub enum AppAction {
    /// Action to trigger a refresh of the UI.
    Refresh,
}

impl Reducible for AppState {
    type Action = AppAction;

    /// Handles state transitions based on dispatched actions.
    ///
    /// # Arguments
    ///
    /// * `self` - The current application state
    /// * `action` - The action to perform on the state
    ///
    /// # Returns
    ///
    /// A new version of the application state after applying the action
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AppAction::Refresh => AppState { refresh_counter: self.refresh_counter + 1 }.into(),
        }
    }
}
