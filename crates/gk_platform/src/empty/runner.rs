use super::Windows;
use gk_app::{App, GKState};

pub fn runner<S: GKState>(mut app: App<S>) -> Result<(), String> {
    app.init();

    loop {
        app.update();

        let request_exit = app
            .get_mut_plugin::<Windows>()
            .unwrap()
            .manager
            .request_exit;

        if request_exit {
            break;
        }
    }

    app.close();

    Ok(())
}
