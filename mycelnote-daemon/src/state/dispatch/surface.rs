use crate::state::State;
use wayland_client::{Connection, Dispatch, QueueHandle, protocol::wl_surface};

impl Dispatch<wl_surface::WlSurface, ()> for State {
    fn event(
        state: &mut State,
        _: &wl_surface::WlSurface,
        event: wl_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<State>,
    ) {
        state.log_event("wl_surface", event);
    }
}
