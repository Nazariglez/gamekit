use crate::{AppBuilder, GKState};

/// Used to set configurations or add plugins to AppBuilder
pub trait BuildConfig<S: GKState> {
    /// Applies the configuration on the app's builder
    fn apply(&self, builder: AppBuilder<S>) -> Result<AppBuilder<S>, String>;

    /// This will delay the evaluation of `apply` just before the apps start and not when is set
    fn late_evaluation(&self) -> bool {
        false
    }
}
