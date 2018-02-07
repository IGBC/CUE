extern crate cursive;

use cursive::{view, Printer, XY};
use cursive::event::*;

use std::thread;
use std::marker;
use std::sync::{Arc, Mutex};

struct WindowContextData {
    //I'll tell you when you're older
}

pub struct ApplicationWindow {
    c: Arc<Mutex<WindowContextData>>, // Mutable data container
}

pub struct WindowView {
    c: Arc<Mutex<WindowContextData>>, // Mutable data container
}

impl ApplicationWindow {
    /// Creates a new Application Window with the given application name and window size
    /// Then spawns the given main function as the app entry point.
    /// The contents of the string parameters are currently undefined.
    pub fn create_app<F>(app_name: &str, size_req: XY<usize>, main: F) -> WindowView
    where
        F: Fn(Vec<&str>, &ApplicationWindow) + 'static + marker::Send,
    {
        // Create inner container that contains all mutable data
        let inner = WindowContextData {
            
        };

        let c = Arc::new(Mutex::new(inner));

        // immutable AF
        let container = ApplicationWindow {
            c: c.clone(),
        };

        // Create IO thread allows updating the buffer without blocking the main thread
        thread::spawn(move || {
            main(Vec::new(), &container);
        });

        WindowView {
            c: c.clone(),
        }
    }

    /// Sets the title text of this window (May be truncated)
    pub fn set_title(&mut self, title: &str) {

    }
    
    /// Works like global callbacks, but are limited to the focus of this app
    /// to prevent hotkey conflicts
    pub fn add_app_callback<F, E: Into<Event>>(&mut self, event: E, cb: F)
    where
        F: Fn(&mut ApplicationWindow) + 'static,
    {
        
    }

    /// Send a user visible notification to the CUE desktop
    pub fn send_notification(content: &str, notify: bool) {
        
    }

    /// Pin a notification to the CUE desktop, call the callback to remove it.
    // pub fn send_sticky_notification(content: &str) -> FnOnce(/*magic to remove identify notification*/) {
    //
    // }

    // Returns True if the window is in focus (May return false positives.)
    pub fn is_focused() -> bool{
        // The documentation makes no promises on accuracy.
        return true;
    }
}