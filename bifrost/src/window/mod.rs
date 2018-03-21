use cursive::backend;
use cursive::theme;
use cursive::view;
use cursive::event::{Callback, Event, EventResult};

pub struct Window<R,B> where
    R: view::View,
    B: backend::Backend,
{
    theme: theme::Theme,
    root_widget: R,
    application_callbacks: HashMap<Event, Vec<Callback>>,
    backend: B,
}

impl Window<R,B> {
    pub fn new() -> Self {

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