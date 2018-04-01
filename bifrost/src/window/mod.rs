use std::collections::HashMap;

use cursive::backend;
use cursive::theme;
use cursive::view;
use cursive::event::{Callback, Event};

pub struct Window {
    theme: theme::Theme,
    root_widget: Box<view::View>,
    application_callbacks: HashMap<Event, Vec<Callback>>,
    backend: Box<backend::Backend>,
}

impl Window {
    //pub fn new() -> Self {
        // 
    // }

    /// Returns the currently used theme.
    pub fn current_theme(&self) -> &theme::Theme {
        &self.theme
    }

    /// Clears the screen.
    ///
    /// Users rarely have to call this directly.
    pub fn clear(&self) {
        self.backend
            .clear(self.theme.palette[theme::PaletteColor::Background]);
    }


}