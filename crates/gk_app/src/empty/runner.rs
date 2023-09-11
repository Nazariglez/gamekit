use crate::app::App;
use gk_sys::{GKState, System};

pub fn runner<S: GKState>(mut app: System<S>) -> Result<(), String> {
    app.init();

    loop {
        app.frame_start();
        app.update();

        let request_exit = app
            .get_mut_plugin::<App>()
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
