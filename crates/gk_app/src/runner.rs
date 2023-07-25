use crate::app::App;
use crate::GKState;

pub(crate) fn default_runner<S: GKState>(mut app: App<S>) -> Result<(), String> {
    // Execute initialize callback
    app.init();

    // Frame starts here
    app.frame_start();

    // Execute event's callback
    app.update();

    // Frame ends here
    app.frame_end();

    // Execute close callback
    app.close();

    Ok(())
}
