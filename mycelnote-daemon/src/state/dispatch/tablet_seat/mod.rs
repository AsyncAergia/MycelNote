use crate::state::State;
use wayland_client::{Connection, Dispatch, QueueHandle, event_created_child};
use wayland_protocols::wp::tablet::zv2::client::{
    zwp_tablet_pad_v2, zwp_tablet_seat_v2, zwp_tablet_tool_v2, zwp_tablet_v2,
};

pub mod pad;
pub mod tablet;
pub mod tool;

impl Dispatch<zwp_tablet_seat_v2::ZwpTabletSeatV2, ()> for State {
    fn event(
        state: &mut State,
        _: &zwp_tablet_seat_v2::ZwpTabletSeatV2,
        event: zwp_tablet_seat_v2::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<State>,
    ) {
        state.log_event("zwp_tablet_seat", event);
    }

    event_created_child!(State, zwp_tablet_seat_v2::ZwpTabletSeatV2, [
        0u16 => (zwp_tablet_v2::ZwpTabletV2, ()),
        1u16 => (zwp_tablet_tool_v2::ZwpTabletToolV2, ()),
        2u16 => (zwp_tablet_pad_v2::ZwpTabletPadV2, ())
    ]);
}
