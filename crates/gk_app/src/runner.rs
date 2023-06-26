use crate::app::App;
use crate::GKState;

pub(crate) fn default_runner<S: GKState>(mut app: App<S>) -> Result<(), String> {
    // Execute initialize callback
    app.initialize();

    // Execute event's callback
    app.tick();

    Ok(())
}
