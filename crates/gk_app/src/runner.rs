use crate::app::App;
use crate::GKState;

pub(crate) fn default_runner<S: GKState>(mut app: App<S>) -> Result<(), String> {
    app.tick();
    Ok(())
}
