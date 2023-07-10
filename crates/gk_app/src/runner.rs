use crate::app::App;
use crate::GKState;

pub(crate) fn default_runner<S: GKState>(mut app: App<S>) -> Result<(), String> {
    // Execute initialize callback
    app.init();

    // Execute event's callback
    app.update();

    // Execute close callback
    app.close();

    Ok(())
}
