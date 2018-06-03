pub mod socketbackend;

use std::collections::HashMap;

use status;

use cursive::{backend, theme, view, views};
use cursive::backend::Backend;
use cursive::event::{Callback, Event};

pub struct Window {
    theme: theme::Theme,
    root_widget: Box<view::View>,
    application_callbacks: HashMap<Event, Vec<Callback>>,
    backend: Box<Backend>,
}

impl Window {
    pub fn new() -> Self {
        if status::is_standalone() {
            return Window::init_standalone();
        } else {
            return Window::init_cue();
        }
    }

    fn init_cue() -> Self {
        let backend = socketbackend::SocketBackend::init();

        let theme = theme::load_default();
        
        Window {
            theme,
            root_widget: views::StackView::new(),
            application_callbacks: HashMap::new(),
            backend,
        } 
    }

    fn init_standalone() -> Self {
        let backend = backend::Concrete::init();
        
        let theme = theme::load_default();
        
        Window {
            theme,
            root_widget: views::StackView::new(),
            application_callbacks: HashMap::new(),
            backend,
        } 
    }

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