use crate::GKWindowId;
use winit::window::WindowId;

pub(crate) fn win_id(window_id: WindowId) -> GKWindowId {
    let raw: u64 = window_id.into();
    raw.into()
}
