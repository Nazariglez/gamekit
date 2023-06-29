use crate::app::App;
use crate::config::BuildConfig;
use crate::handlers::{
    Handler, PluginHandler, RunnerHandlerFn, SetupHandler, SetupHandlerFn, UpdateHandlerFn,
};
use crate::runner::default_runner;
use crate::storage::{Plugins, Storage};
use crate::{GKState, Plugin};
use indexmap::IndexMap;

pub struct AppBuilder<S: GKState + 'static> {
    plugins: Plugins,
    runner: Box<RunnerHandlerFn<S>>,
    setup_handler: Box<SetupHandlerFn<S>>,
    init_handler: Box<UpdateHandlerFn<S>>,
    event_handler: Box<UpdateHandlerFn<S>>,
    close_handler: Box<UpdateHandlerFn<S>>,
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
        let init_handler: Box<UpdateHandlerFn<S>> = Box::new(|_| {});
        let event_handler: Box<UpdateHandlerFn<S>> = Box::new(|_| {});
        let close_handler: Box<UpdateHandlerFn<S>> = Box::new(|_| {});
        let late_configs = Some(Default::default());

        Self {
            plugins,
            runner,
            setup_handler,
            init_handler,
            event_handler,
            close_handler,
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

    pub fn on_init<T, H>(mut self, mut handler: H) -> Self
    where
        H: Handler<S, T> + 'static,
    {
        self.init_handler = Box::new(move |storage| handler.call(storage));
        self
    }

    pub fn on_event<T, H>(mut self, mut handler: H) -> Self
    where
        H: Handler<S, T> + 'static,
    {
        self.event_handler = Box::new(move |storage| handler.call(storage));
        self
    }

    pub fn on_close<T, H>(mut self, mut handler: H) -> Self
    where
        H: Handler<S, T> + 'static,
    {
        self.close_handler = Box::new(move |storage| handler.call(storage));
        self
    }

    pub fn with_runner<F: FnMut(App<S>) -> Result<(), String> + 'static>(
        mut self,
        runner: F,
    ) -> Self {
        self.runner = Box::new(runner);
        self
    }

    pub fn add_plugin<T: Plugin + 'static>(mut self, plugin: T) -> Self {
        self.plugins.add(plugin);
        self
    }

    pub fn add_plugin_with<T, P, H>(mut self, mut handler: H) -> Result<Self, String>
    where
        T: 'static,
        P: Plugin + 'static,
        H: PluginHandler<P, T> + 'static,
    {
        let plugin = handler.call(&mut self.plugins)?;
        Ok(self.add_plugin(plugin))
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
            events: Default::default(),
            update_handler: event_handler,
            initialized: false,
        };

        (runner)(app)?;

        Ok(())
    }
}
