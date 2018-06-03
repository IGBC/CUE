use cursive::backend::Backend;
use cursive::{event, theme};

#[derive(Debug)]
pub struct SocketBackend {
    current_color: theme::ColorPair,

}

impl Backend for SocketBackend {
    fn init() -> Box<Self> where Self: Sized {
        let c = SocketBackend {
            current_color: theme::ColorPair {
                front: theme::Color::TerminalDefault, 
                back: theme::Color::TerminalDefault
            }
        };
        Box::new(c)
    }

    // TODO: take `self` by value?
    // Or implement Drop?
    fn finish(&mut self) {

    }

    fn refresh(&mut self) {

    }

    fn has_colors(&self) -> bool {
        true
    }
    fn screen_size(&self) -> (usize, usize) {
        (0, 0)
    }

    /// In most backends this is the main event loop.
    /// Don't call this function. It doesn't work like that with
    /// the network backend. 
    /// 
    /// Events are pushed up from the CUE socket, not polled. Build a
    /// handler that responds to the calback floating up.
    fn poll_event(&mut self) -> event::Event {
        panic!("Attempted call to NetworkBackend::poll_event. RTFM!");
    }

    /// Main method used for printing
    fn print_at(&self, (x, y): (usize, usize), text: &str) {

    }

    fn clear(&self, color: theme::Color) {

    }

    fn set_refresh_rate(&mut self, fps: u32) {
        // We can't, the refresh rate is actually controlled
        // by the CUE Compositor, so we'll lie.

        {}

        // Yea I totally did that 

        return;
    }

    // This sets the Colours and returns the previous colours
    // to allow you to set them back when you're done.
    fn set_color(&self, colors: theme::ColorPair) -> theme::ColorPair {
        let current = self.current_color;
        current = colors;
        return current;
    }

    fn set_effect(&self, effect: theme::Effect) {

    }


    fn unset_effect(&self, effect: theme::Effect) {
        
    }
}