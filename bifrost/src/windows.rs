extern crate cursive;

use self::cursive::event::Event;
use self::cursive::vec::Vec2;

#[derive(Debug)]
struct WindowContext {
    
}

impl WindowContext {
    /// Opens new window in CUE desktop
    /// TBD: how to sanely handle this in standalone mode.
    pub fn create_window(size_req: Vec2) -> Self {
        panic!("Not Implemented");
    }

    /// Change window title.
    /// Changes Xterm window title in standalone mode.
    pub fn set_title(title: &str) {
        panic!("Not Implemented");
    }

    /// Add application wide event listener.
    /// Event listener is only active while application is in focus.
    /// Returns true with sucessful registration.
    /// Returns false with an event conflict.
    pub fn add_application_callback<F, E: Into<Event>>(&mut self, event: E, cb: F) -> bool
        where
            F: Fn(&mut WindowContext) + 'static, {
        panic!("Not Implemented");
    }

    /// Sends notification to CUE desktop tray.
    /// Has no effect in standalone mode.
    pub fn send_notification(text: &str, notify: bool) {
        panic!("Not Implemented");
    }

    //fn send_sticky_notify(&str: text) {}

    /// Returns if application root is ready to recieve events;
    /// may return false positives
    pub fn is_focused() -> bool {
        // we made no promises of accuracy.
        true
    }
}
