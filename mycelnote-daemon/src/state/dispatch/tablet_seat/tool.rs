use crate::state::State;
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_protocols::wp::tablet::zv2::client::zwp_tablet_tool_v2;

impl Dispatch<zwp_tablet_tool_v2::ZwpTabletToolV2, ()> for State {
    fn event(
        state: &mut State,
        tool: &zwp_tablet_tool_v2::ZwpTabletToolV2,
        event: zwp_tablet_tool_v2::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<State>,
    ) {
        match event {
            zwp_tablet_tool_v2::Event::ProximityIn { serial, .. } => {
                if let Some(ref surface) = state.cursor_surface {
                    tool.set_cursor(serial, Some(surface.as_ref()), 0, 0);
                }
            }
            zwp_tablet_tool_v2::Event::Down { serial } => {
                if let Some(ref surface) = state.cursor_surface {
                    tool.set_cursor(serial, Some(surface.as_ref()), 0, 0);
                }
            }
            _ => {}
        }
        state.log_event("zwp_tablet_tool", event);
    }
}
