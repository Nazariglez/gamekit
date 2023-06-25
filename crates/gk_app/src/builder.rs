use crate::app::App;
use crate::config::BuildConfig;
use crate::handlers::{EventHandlerFn, Handler, RunnerHandlerFn, SetupHandler, SetupHandlerFn};
use crate::runner::default_runner;
use crate::storage::{Plugins, Storage};
use crate::GKState;
use indexmap::IndexMap;

pub struct AppBuilder<S: GKState + 'static> {
    plugins: Plugins,
    runner: Box<RunnerHandlerFn<S>>,
    setup_handler: Box<SetupHandlerFn<S>>,
    event_handler: Box<EventHandlerFn<S>>,
    late_configs: Option<IndexMap<std::any::TypeId, Box<dyn BuildConfig<S>>>>,
}

impl GKState for () {}

impl AppBuilder<()> {
    pub fn init() -> Self {
        Self::init_with(|| Ok(()))
    }
}

impl<S: GKState> AppBuilder<S> {
    pub fn init_with<T, H>(handler: H) -> Self
    where
        H: SetupHandler<S, T> + 'static,
    {
        let mut plugins = Plugins::new();
        let runner = Box::new(default_runner);
        let setup_handler: Box<SetupHandlerFn<S>> = Box::new(|plugins| handler.call(plugins));
        let event_handler: Box<EventHandlerFn<S>> = Box::new(|_| {});
        let late_configs = Some(Default::default());

        Self {
            plugins,
            runner,
            setup_handler,
            event_handler,
            late_configs,
        }
    }

    pub fn add_config<C>(mut self, config: C) -> Result<Self, String>
    where
        C: BuildConfig<S> + 'static,
    {
        if config.late_evaluation() {
            if let Some(late_configs) = &mut self.late_configs {
                let typ = std::any::TypeId::of::<C>();
                late_configs.insert(typ, Box::new(config));
            }

            return Ok(self);
        }

        config.apply(self)
    }

    pub fn set_event<T, H>(mut self, mut handler: H) -> Self
    where
        H: Handler<S, T> + 'static,
    {
        self.event_handler = Box::new(move |storage| handler.call(storage));
        self
    }

    pub fn set_runner<F: FnMut(App<S>) -> Result<(), String> + 'static>(
        mut self,
        runner: F,
    ) -> Self {
        self.runner = Box::new(runner);
        self
    }

    pub fn add_plugin<T: 'static>(mut self, plugin: T) -> Self {
        self.plugins.add(plugin);
        self
    }

    pub fn build(mut self) -> Result<(), String> {
        if let Some(late_configs) = self.late_configs.take() {
            for (_, config) in late_configs {
                self = config.apply(self)?;
            }
        }

        let Self {
            mut plugins,
            mut runner,
            setup_handler,
            event_handler,
            ..
        } = self;

        let state = (setup_handler)(&mut plugins)?;
        let storage = Storage { plugins, state };

        let app = App {
            storage,
            events: vec![],
            event_handler,
        };

        (runner)(app)?;

        Ok(())
    }
}
