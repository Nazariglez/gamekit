use crate::platform::Platform;
use gk_app::{App, GKState};

pub fn runner<S: GKState>(mut app: App<S>) -> Result<(), String> {
    app.init();

    loop {
        app.frame_start();
        app.update();
        app.draw();

        let request_exit = app
            .get_mut_plugin::<Platform>()
            .ok_or_else(|| "Cannot find Platform plugin.")?
            .manager
            .request_exit;

        app.frame_end();

        if request_exit {
            break;
        }
    }

    app.close();

    Ok(())
}
