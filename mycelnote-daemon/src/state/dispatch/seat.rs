use crate::state::State;
use wayland_client::{Connection, Dispatch, QueueHandle, protocol::wl_seat};

impl Dispatch<wl_seat::WlSeat, ()> for State {
    fn event(
        state: &mut State,
        _: &wl_seat::WlSeat,
        event: wl_seat::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<State>,
    ) {
        state.log_event("wl_seat", event);
    }
}
