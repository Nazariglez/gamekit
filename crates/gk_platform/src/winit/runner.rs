use super::utils::win_id;
use super::Windows;
use crate::window::{GKWindow, WindowEvent, WindowEventId};
use gk_app::{App, GKState};
use winit::event::{Event, WindowEvent as WWindowEvent};

pub fn runner<S: GKState + 'static>(mut app: App<S>) -> Result<(), String> {
    let event_loop = app
        .get_mut_plugin::<Windows>()
        .ok_or_else(|| "Cannot find Windows plugin.")?
        .manager
        .event_loop
        .take()
        .ok_or_else(|| "Something went wrong acquiring the Winit's EventLoop.".to_string())?;

    let mut initialized = false;
    event_loop.run(move |evt, event_loop, control_flow| {
        app.get_mut_plugin::<Windows>()
            .unwrap()
            .manager
            .event_loop
            .set(event_loop);

        control_flow.set_wait();
        println!("{evt:?}");

        match evt {
            Event::Resumed => {
                // init the app's logic on the first resumed event
                if !initialized {
                    initialized = true;
                    app.init();
                }
            }
            Event::LoopDestroyed => {
                app.close();
            }
            Event::WindowEvent { window_id, event } => {
                let windows = app.get_mut_plugin::<Windows>().unwrap();
                let id = win_id(window_id);
                if let Some(win) = windows.window(id) {
                    match event {
                        WWindowEvent::Resized(size) => {
                            let size = size.to_logical::<u32>(win.scale());
                            app.event(WindowEvent {
                                id,
                                event: WindowEventId::Resized {
                                    width: size.width,
                                    height: size.height,
                                },
                            });
                        }
                        WWindowEvent::Moved(pos) => {
                            let pos = pos.to_logical::<i32>(win.scale());
                            app.event(WindowEvent {
                                id,
                                event: WindowEventId::Moved { x: pos.x, y: pos.y },
                            });
                        }
                        WWindowEvent::CloseRequested => {
                            let windows = app.get_mut_plugin::<Windows>().unwrap();
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
                        WWindowEvent::ScaleFactorChanged { .. } => {}
                        WWindowEvent::ThemeChanged(_) => {}
                        WWindowEvent::Occluded(_) => {}
                    }
                }
            }
            Event::MainEventsCleared => {
                app.update();
            }
            _ => (),
        }

        let manager = &mut app.get_mut_plugin::<Windows>().unwrap().manager;
        manager.event_loop.unset();
        if manager.request_exit {
            control_flow.set_exit();
        }
    });
}
