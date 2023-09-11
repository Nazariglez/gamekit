use super::utils::win_id;
use crate::App;
use gk_sys::window::{GKWindow, GKWindowId, WindowEvent, WindowEventId};
use gk_sys::{GKState, System};
use hashbrown::HashSet;
use winit::event::{Event, WindowEvent as WWindowEvent};

pub fn runner<S: GKState + 'static>(mut app: System<S>) -> Result<(), String> {
    let event_loop = app
        .get_mut_plugin::<App>()
        .ok_or_else(|| "Cannot find Windows plugin.")?
        .manager
        .event_loop
        .take()
        .ok_or_else(|| "Something went wrong acquiring the Winit's EventLoop.".to_string())?;

    let mut initialized_app = false;

    // Send initialize event if this is a new window
    let mut initialized_windows = HashSet::new();
    let mut init_window = move |id: GKWindowId, app: &mut System<S>| {
        if !initialized_windows.contains(&id) {
            initialized_windows.insert(id);
            app.event(WindowEvent {
                id,
                event: WindowEventId::Init,
            });
        }
    };

    event_loop.run(move |evt, event_loop, control_flow| {
        app.get_mut_plugin::<App>()
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
                    app.init();
                }
            }
            Event::NewEvents(_) => {
                app.frame_start();
            }
            Event::RedrawEventsCleared => {
                app.frame_end();
            }
            Event::MainEventsCleared => {
                app.update();
            }
            Event::RedrawRequested(id) => {
                let id = win_id(id);

                // Sometimes this event comes before any WindowEvent
                // Initializing windows here too we avoid a first blank frame
                init_window(id, &mut app);

                app.draw(id);
            }
            Event::LoopDestroyed => {
                app.close();
            }

            // -- Windowing events
            Event::WindowEvent { window_id, event } => {
                let windows = app.get_mut_plugin::<App>().unwrap();
                let id = win_id(window_id);
                if let Some(win) = windows.window(id) {
                    let scale_factor = win.scale();
                    init_window(id, &mut app);

                    match event {
                        WWindowEvent::Resized(size) => {
                            let size = size.to_logical::<u32>(scale_factor);
                            app.event(WindowEvent {
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
                            app.event(WindowEvent {
                                id,
                                event: WindowEventId::Moved { x: pos.x, y: pos.y },
                            });
                        }
                        WWindowEvent::CloseRequested => {
                            let windows = app.get_mut_plugin::<App>().unwrap();
                            windows.close(id);
                            app.event(WindowEvent {
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
                            app.event(WindowEvent {
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
                            app.event(WindowEvent {
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

        let manager = &mut app.get_mut_plugin::<App>().unwrap().manager;
        manager.event_loop.unset();
        if manager.request_exit {
            control_flow.set_exit();
        }
    });
}
