use crate::state::State;
use wayland_client::{Connection, Dispatch, QueueHandle, event_created_child};
use wayland_protocols::wp::tablet::zv2::client::{zwp_tablet_pad_group_v2, zwp_tablet_pad_v2};

pub mod group;

impl Dispatch<zwp_tablet_pad_v2::ZwpTabletPadV2, ()> for State {
    fn event(
        state: &mut State,
        _: &zwp_tablet_pad_v2::ZwpTabletPadV2,
        event: zwp_tablet_pad_v2::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<State>,
    ) {
        state.log_event("zwp_tablet_pad", event);
    }

    event_created_child!(State, zwp_tablet_pad_v2::ZwpTabletPadV2, [
        0u16 => (zwp_tablet_pad_group_v2::ZwpTabletPadGroupV2, ()),
    ]);
}
