use crate::state::State;
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_protocols::wp::tablet::zv2::client::zwp_tablet_v2;

impl Dispatch<zwp_tablet_v2::ZwpTabletV2, ()> for State {
    fn event(
        state: &mut State,
        _: &zwp_tablet_v2::ZwpTabletV2,
        event: zwp_tablet_v2::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<State>,
    ) {
        state.log_event("zwp_tablet", event);
    }
}
