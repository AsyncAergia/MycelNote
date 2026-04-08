use wayland_client::{Connection, EventQueue, globals::registry_queue_init, protocol::wl_seat};
use wayland_protocols::wp::tablet::zv2::client::zwp_tablet_manager_v2;

use crate::state::State;

/// Manages the Wayland compositor connection and tablet manager bindings
pub struct WaylandClient {
    pub connection: Connection,
    pub queue: EventQueue<State>,
}

impl WaylandClient {
    /// Initialize connection to Wayland compositor and bind to tablet protocol
    pub fn connect() -> Result<Self, Box<dyn std::error::Error>> {
        let connection = Connection::connect_to_env()?;
        let (globals, queue) = registry_queue_init::<State>(&connection)?;

        // Bind to tablet manager
        let tablet_manager = globals.bind::<zwp_tablet_manager_v2::ZwpTabletManagerV2, _, _>(
            &queue.handle(),
            1..=1,
            (),
        )?;

        // Bind to seat
        let _wl_seat = globals.bind::<wl_seat::WlSeat, _, _>(&queue.handle(), 1..=10, ())?;
        let _tablet_seat = tablet_manager.get_tablet_seat(&_wl_seat, &queue.handle(), ());

        Ok(WaylandClient { connection, queue })
    }

    /// Dispatch pending events from Wayland
    pub fn dispatch(&mut self, state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
        self.queue.blocking_dispatch(state)?;
        Ok(())
    }
}
