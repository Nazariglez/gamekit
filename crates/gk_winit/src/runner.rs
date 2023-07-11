use crate::utils::win_id;
use crate::Manager;
use gk_app::window::{
    GKWindow, GKWindowManager, WindowEvent as GkWindowEvent, WindowEventId as GkEvent,
};
use gk_app::{App, GKState};
use winit::event::{Event, WindowEvent};

pub fn runner<S: GKState + 'static>(mut app: App<S>) -> Result<(), String> {
    let event_loop = app
        .get_mut_plugin::<Manager>()
        .ok_or_else(|| "Cannot find Winit's Window Manager".to_string())?
        .event_loop
        .take()
        .ok_or_else(|| "Something went wrong acquiring the Winit's EventLoop.".to_string())?;

    let mut initialized = false;

    event_loop.run(move |evt, event_loop, control_flow| {
        app.get_mut_plugin::<Manager>()
            .unwrap()
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
                let manager = app.get_mut_plugin::<Manager>().unwrap();
                let id = win_id(window_id);
                if let Some(win) = manager.window(id) {
                    match event {
                        WindowEvent::Resized(size) => {
                            let size = size.to_logical::<u32>(win.scale());
                            app.event(GkWindowEvent {
                                id,
                                event: GkEvent::Resized {
                                    width: size.width,
                                    height: size.height,
                                },
                            });
                        }
                        WindowEvent::Moved(pos) => {
                            let pos = pos.to_logical::<i32>(win.scale());
                            app.event(GkWindowEvent {
                                id,
                                event: GkEvent::Moved { x: pos.x, y: pos.y },
                            });
                        }
                        WindowEvent::CloseRequested => {
                            app.event(GkWindowEvent {
                                id,
                                event: GkEvent::CloseRequest,
                            });
                        }
                        WindowEvent::Destroyed => {}
                        WindowEvent::DroppedFile(_) => {}
                        WindowEvent::HoveredFile(_) => {}
                        WindowEvent::HoveredFileCancelled => {}
                        WindowEvent::ReceivedCharacter(_) => {}
                        WindowEvent::Focused(focus) => {
                            app.event(GkWindowEvent {
                                id,
                                event: if focus {
                                    GkEvent::FocusGained
                                } else {
                                    GkEvent::FocusLost
                                },
                            });
                        }
                        WindowEvent::KeyboardInput { .. } => {}
                        WindowEvent::ModifiersChanged(_) => {}
                        WindowEvent::Ime(_) => {}
                        WindowEvent::CursorMoved { .. } => {}
                        WindowEvent::CursorEntered { .. } => {}
                        WindowEvent::CursorLeft { .. } => {}
                        WindowEvent::MouseWheel { .. } => {}
                        WindowEvent::MouseInput { .. } => {}
                        WindowEvent::TouchpadMagnify { .. } => {}
                        WindowEvent::SmartMagnify { .. } => {}
                        WindowEvent::TouchpadRotate { .. } => {}
                        WindowEvent::TouchpadPressure { .. } => {}
                        WindowEvent::AxisMotion { .. } => {}
                        WindowEvent::Touch(_) => {}
                        WindowEvent::ScaleFactorChanged { .. } => {}
                        WindowEvent::ThemeChanged(_) => {}
                        WindowEvent::Occluded(_) => {}
                    }
                }
            }
            // Event::WindowEvent {
            //     window_id,
            //     event: WindowEvent::CloseRequested,
            // } => {
            //     let raw_id: u64 = window_id.into();
            //     let id: GKWindowId = raw_id.into();
            //
            //     let manager = app.get_mut_plugin::<Manager>().unwrap();
            //     manager.windows.remove(&id);
            //
            //     if manager.windows.is_empty() {
            //         manager.request_exit = true;
            //         println!("Close");
            //     }
            // }
            Event::MainEventsCleared => {
                app.update();
            }
            _ => (),
        }

        let manager = app.get_mut_plugin::<Manager>().unwrap();
        manager.event_loop.unset();
        if manager.request_exit {
            control_flow.set_exit();
        }
    });

    Ok(())
}
