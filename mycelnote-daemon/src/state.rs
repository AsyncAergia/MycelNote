mod dispatch;
use std::sync::Arc;
use wayland_client::protocol::wl_surface::WlSurface;

pub struct State {
    pub cursor_surface: Option<Arc<WlSurface>>,
}

impl State {
    pub fn new() -> Self {
        State {
            cursor_surface: None,
        }
    }

    pub fn log_event<T: std::fmt::Debug>(&self, name: &str, event: T) {
        println!("{}: {:?}", name, event);
    }
}
