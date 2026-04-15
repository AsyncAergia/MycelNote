use wayland_client::{
    Connection, EventQueue,
    globals::registry_queue_init,
    protocol::{wl_compositor, wl_seat, wl_surface},
};
use wayland_protocols::wp::tablet::zv2::client::{zwp_tablet_manager_v2, zwp_tablet_seat_v2};

use crate::state::State;

/// Manages the Wayland compositor connection and tablet manager bindings
pub struct WaylandClient {
    #[allow(dead_code)]
    connection: Connection,
    queue: EventQueue<State>,
    tablet_seat: zwp_tablet_seat_v2::ZwpTabletSeatV2,
    pub surface_stylus: wl_surface::WlSurface,
}

impl WaylandClient {
    /// Initialize connection to Wayland compositor and bind to tablet protocol
    pub fn connect() -> Result<Self, Box<dyn std::error::Error>> {
        let connection = Connection::connect_to_env()?;
        let (globals, queue) = registry_queue_init::<State>(&connection)?;

        // Bind to compositor to create surfaces
        let compositor =
            globals.bind::<wl_compositor::WlCompositor, _, _>(&queue.handle(), 1..=5, ())?;

        // Create a surface for the stylus cursor
        let surface_stylus = compositor.create_surface(&queue.handle(), ());
        surface_stylus.commit();

        // Bind to tablet manager
        let _tablet_manager = globals.bind::<zwp_tablet_manager_v2::ZwpTabletManagerV2, _, _>(
            &queue.handle(),
            1..=2,
            (),
        )?;

        // Bind to seat
        let _wl_seat = globals.bind::<wl_seat::WlSeat, _, _>(&queue.handle(), 1..=10, ())?;
        let tablet_seat = _tablet_manager.get_tablet_seat(&_wl_seat, &queue.handle(), ());

        Ok(WaylandClient {
            connection,
            queue,
            tablet_seat,
            surface_stylus,
        })
    }

    /// Dispatch pending events from Wayland
    pub fn dispatch(&mut self, state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
        self.queue.blocking_dispatch(state)?;
        Ok(())
    }

    pub fn get_surface(&self) -> &wl_surface::WlSurface {
        &self.surface_stylus
    }
}
