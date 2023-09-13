use gk_sys::mouse::{MouseAction, MouseEvent};
use gk_sys::window::WindowId;
use winit::event::{ElementState, MouseButton as WMouseButton};

pub(crate) fn process_input(
    window_id: WindowId,
    state: ElementState,
    btn: WMouseButton,
    pos: Option<(f32, f32)>,
) -> MouseEvent {
    todo!()
    // let button = button_id(btn);
    // match state {
    //     ElementState::Pressed => MouseEvent {
    //         window_id,
    //         input: MouseInput::ButtonPressed { button },
    //     },
    //     ElementState::Released => {}
    // }
}

pub(crate) fn process_motion(
    window_id: WindowId,
    pos: (f32, f32),
    old: Option<(f32, f32)>,
) -> MouseEvent {
    todo!()
}

pub(crate) fn process_wheel(window_id: WindowId) {}
