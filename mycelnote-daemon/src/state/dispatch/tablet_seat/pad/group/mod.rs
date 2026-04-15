use crate::state::State;
use wayland_client::{Connection, Dispatch, QueueHandle, event_created_child};
use wayland_protocols::wp::tablet::zv2::client::{
    zwp_tablet_pad_dial_v2, zwp_tablet_pad_group_v2, zwp_tablet_pad_ring_v2,
    zwp_tablet_pad_strip_v2, zwp_tablet_pad_v2,
};

pub mod dial;
pub mod ring;
pub mod strip;

impl Dispatch<zwp_tablet_pad_group_v2::ZwpTabletPadGroupV2, ()> for State {
    fn event(
        state: &mut State,
        _: &zwp_tablet_pad_group_v2::ZwpTabletPadGroupV2,
        event: zwp_tablet_pad_group_v2::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<State>,
    ) {
        state.log_event("zwp_tablet_pad_group", event);
    }

    event_created_child!(State, zwp_tablet_pad_v2::ZwpTabletPadV2, [
        1u16 => (zwp_tablet_pad_ring_v2::ZwpTabletPadRingV2, ()),
        2u16 => (zwp_tablet_pad_strip_v2::ZwpTabletPadStripV2, ()),
        6u16 => (zwp_tablet_pad_dial_v2::ZwpTabletPadDialV2, ())
    ]);
}
