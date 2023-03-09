use std::sync::mpsc::Receiver;

use glfw::{Glfw, Window, WindowMode, WindowEvent};

pub fn debug_string(text: &str) {
    println!("[DEBUG] {}", text);
}

pub struct LWL {
    glfw: Glfw,
    _events: Option<(Window, Receiver<(f64, WindowEvent)>)>,
    window: Window
}

impl LWL {

    pub fn init_window(width: i32, height: i32, title: &str) -> LWL {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let mut win_events = glfw.create_window(width as u32, height as u32, title, WindowMode::Windowed);
        let window = win_events.take().unwrap().0;
        return LWL {glfw, _events: win_events, window};
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }
}
