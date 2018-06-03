use super::window::Window;

pub struct BiFrost {
    window_list: Vec<Window>,
}

impl BiFrost {
    pub fn create_window(&mut self) {
        self.window_list.push(Window::new());
    }
}