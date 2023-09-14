use super::utils::win_id;
use crate::winit::{keyboard, mouse};
use crate::App;
use gk_sys::mouse::MouseEvent;
use gk_sys::window::{GKWindow, WindowEvent, WindowEventId, WindowId};
use gk_sys::{GKState, System};
use hashbrown::{HashMap, HashSet};
use winit::event::{Event, WindowEvent as WWindowEvent};

#[derive(Default)]
struct InnerWindowList(HashMap<WindowId, InnerWindowData>);

impl InnerWindowList {
    fn init_window<S: GKState + 'static>(&mut self, id: WindowId, sys: &mut System<S>) {
        if !self.0.contains_key(&id) {
            self.0.insert(
                id,
                InnerWindowData {
                    id,
                    mouse_pos: None,
                },
            );
            sys.event(WindowEvent {
                id,
                event: WindowEventId::Init,
            });
        }
    }

    fn remove(&mut self, id: &WindowId) {
        self.0.remove(id);
    }

    fn mouse_pos(&self, id: &WindowId) -> Option<(f32, f32)> {
        self.0.get(id).and_then(|inner| inner.mouse_pos)
    }

    fn set_mouse_pos(&mut self, id: &WindowId, pos: Option<(f32, f32)>) {
        if let Some(win) = self.0.get_mut(id) {
            win.mouse_pos = pos;
        }
    }
}

struct InnerWindowData {
    id: WindowId,
    mouse_pos: Option<(f32, f32)>,
}

pub fn runner<S: GKState + 'static>(mut sys: System<S>) -> Result<(), String> {
    let event_loop = sys
        .get_mut_plugin::<App>()
        .ok_or("Cannot find Windows plugin.")?
        .manager
        .event_loop
        .take()
        .ok_or("Something went wrong acquiring the Winit's EventLoop.")?;

    let mut initialized_app = false;

    // track some inner data
    let mut inner_window_list = InnerWindowList::default();
    event_loop.run(move |evt, event_loop, control_flow| {
        sys.get_mut_plugin::<App>()
            .unwrap()
            .manager
            .event_loop
            .set(event_loop);

        control_flow.set_poll();
        // control_flow.set_wait();
        // println!("{evt:?}");
        match evt {
            // -- App life cycle events
            Event::Resumed => {
                // init the app's logic on the first resumed event
                if !initialized_app {
                    initialized_app = true;
                    sys.init();
                }
            }
            Event::NewEvents(_) => {
                sys.frame_start();
            }
            Event::RedrawEventsCleared => {
                sys.frame_end();
            }
            Event::MainEventsCleared => {
                sys.update();
            }
            Event::RedrawRequested(id) => {
                let id = win_id(id);

                // Sometimes this event comes before any WindowEvent
                // Initializing windows here too we avoid a first blank frame
                inner_window_list.init_window(id, &mut sys);

                sys.draw(id);
            }
            Event::LoopDestroyed => {
                sys.close();
            }

            // -- Windowing events
            Event::WindowEvent { window_id, event } => {
                let windows = sys.get_mut_plugin::<App>().unwrap();
                let id = win_id(window_id);
                if let Some(win) = windows.window(id) {
                    let scale_factor = win.scale();
                    inner_window_list.init_window(id, &mut sys);

                    match event {
                        // keyboard events
                        WWindowEvent::KeyboardInput { input, .. } => {
                            let evt = keyboard::process(id, input);
                            sys.event(evt);
                        }

                        // mouse events
                        WWindowEvent::MouseInput { state, button, .. } => {
                            let evt = mouse::process_input(
                                id,
                                state,
                                button,
                                inner_window_list.mouse_pos(&id),
                            );
                            sys.event(evt);
                        }
                        WWindowEvent::MouseWheel { delta, .. } => {
                            let evt = mouse::process_wheel(
                                id,
                                delta,
                                scale_factor,
                                inner_window_list.mouse_pos(&id),
                            );
                            sys.event(evt);
                        }
                        WWindowEvent::CursorMoved { position, .. } => {
                            let pos = position.to_logical::<f64>(scale_factor);
                            let evt = mouse::process_motion(
                                id,
                                pos.into(),
                                inner_window_list.mouse_pos(&id),
                            );
                            inner_window_list.set_mouse_pos(&id, Some((pos.x as _, pos.y as _)));
                            sys.event(evt);
                        }
                        WWindowEvent::CursorEntered { .. } => {
                            let evt = mouse::process_enter(id, inner_window_list.mouse_pos(&id));
                            sys.event(evt);
                        }
                        WWindowEvent::CursorLeft { .. } => {
                            let evt = mouse::process_leave(id, inner_window_list.mouse_pos(&id));
                            sys.event(evt);
                        }

                        // window events
                        WWindowEvent::Resized(size) => {
                            let size = size.to_logical::<u32>(scale_factor);
                            sys.event(WindowEvent {
                                id,
                                event: WindowEventId::Resized {
                                    width: size.width,
                                    height: size.height,
                                    scale_factor,
                                },
                            });
                        }
                        WWindowEvent::Moved(pos) => {
                            let pos = pos.to_logical::<i32>(scale_factor);
                            sys.event(WindowEvent {
                                id,
                                event: WindowEventId::Moved { x: pos.x, y: pos.y },
                            });
                        }
                        WWindowEvent::CloseRequested => {
                            let windows = sys.get_mut_plugin::<App>().unwrap();
                            windows.close(id);
                            sys.event(WindowEvent {
                                id,
                                event: WindowEventId::Close,
                            });
                        }
                        WWindowEvent::Destroyed => {}
                        WWindowEvent::DroppedFile(_) => {}
                        WWindowEvent::HoveredFile(_) => {}
                        WWindowEvent::HoveredFileCancelled => {}
                        WWindowEvent::ReceivedCharacter(_) => {}
                        WWindowEvent::Focused(focus) => {
                            sys.event(WindowEvent {
                                id,
                                event: if focus {
                                    WindowEventId::FocusGained
                                } else {
                                    WindowEventId::FocusLost
                                },
                            });
                        }
                        WWindowEvent::KeyboardInput { .. } => {}
                        WWindowEvent::ModifiersChanged(_) => {}
                        WWindowEvent::Ime(_) => {}
                        WWindowEvent::CursorMoved { .. } => {}
                        WWindowEvent::CursorEntered { .. } => {}
                        WWindowEvent::CursorLeft { .. } => {}
                        WWindowEvent::MouseWheel { .. } => {}
                        WWindowEvent::MouseInput { .. } => {}
                        WWindowEvent::TouchpadMagnify { .. } => {}
                        WWindowEvent::SmartMagnify { .. } => {}
                        WWindowEvent::TouchpadRotate { .. } => {}
                        WWindowEvent::TouchpadPressure { .. } => {}
                        WWindowEvent::AxisMotion { .. } => {}
                        WWindowEvent::Touch(_) => {}
                        WWindowEvent::ScaleFactorChanged {
                            scale_factor,
                            new_inner_size,
                        } => {
                            let size = new_inner_size.to_logical::<u32>(scale_factor);
                            sys.event(WindowEvent {
                                id,
                                event: WindowEventId::Resized {
                                    width: size.width,
                                    height: size.height,
                                    scale_factor,
                                },
                            });
                        }
                        WWindowEvent::ThemeChanged(_) => {}
                        WWindowEvent::Occluded(_) => {}
                    }
                }
            }
            _ => (),
        }

        let manager = &mut sys.get_mut_plugin::<App>().unwrap().manager;
        manager.event_loop.unset();
        if manager.request_exit {
            control_flow.set_exit();
        }
    });
}
