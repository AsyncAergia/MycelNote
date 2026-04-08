mod state;
mod wayland_client;

use state::State;
use wayland_client::WaylandClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Wayland connection and bind to tablet protocol
    let mut wayland = WaylandClient::connect()?;
    let mut state = State::new();

    println!("Connected to Wayland compositor. Listening for tablet events...");

    loop {
        wayland.dispatch(&mut state)?;
    }
}
