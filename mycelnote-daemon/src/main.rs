mod state;
mod wayland_connection;

use state::State;
use wayland_connection::WaylandClient;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Wayland connection and bind to tablet protocol
    let mut wayland = WaylandClient::connect()?;
    let mut state = State::new();
    
    // Store cursor surface for tool dispatch
    state.cursor_surface = Some(Arc::new(wayland.get_surface().clone()));

    println!("Connected to Wayland compositor. Listening for tablet events...");

    loop {
        wayland.dispatch(&mut state)?;
    }
}

