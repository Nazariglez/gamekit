use crate::app::App;
use crate::config::BuildConfig;
use crate::event::{EventMap, EventQueue};
use crate::handlers::{
    EventHandler, EventHandlerFn, Handler, PluginHandler, RunnerHandlerFn, SetupHandler,
    SetupHandlerFn, UpdateHandlerFn,
};
use crate::runner::default_runner;
use crate::storage::{Plugins, Storage};
use crate::{GKState, Plugin};
use indexmap::IndexMap;
use std::any::TypeId;
use std::collections::HashMap;

pub struct AppBuilder<S: GKState + 'static> {
    plugins: Plugins,
    runner: Box<RunnerHandlerFn<S>>,
    setup_handler: Box<SetupHandlerFn<S>>,
    init_handler: Box<UpdateHandlerFn<S>>,
    update_handler: Box<UpdateHandlerFn<S>>,
    event_handler: EventMap,
    close_handler: Box<UpdateHandlerFn<S>>,
    late_configs: Option<IndexMap<TypeId, Box<dyn BuildConfig<S>>>>,
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
        let plugins = Plugins::new();
        let runner = Box::new(default_runner);
        let setup_handler: Box<SetupHandlerFn<S>> = Box::new(|plugins| handler.call(plugins));
        let init_handler: Box<UpdateHandlerFn<S>> = Box::new(|_| {});
        let update_handler: Box<UpdateHandlerFn<S>> = Box::new(|_| {});
        let close_handler: Box<UpdateHandlerFn<S>> = Box::new(|_| {});
        let event_handler = HashMap::default();
        let late_configs = Some(Default::default());

        Self {
            plugins,
            runner,
            setup_handler,
            init_handler,
            event_handler,
            update_handler,
            close_handler,
            late_configs,
        }
    }

    pub fn add_config<C>(mut self, mut config: C) -> Result<Self, String>
    where
        C: BuildConfig<S> + 'static,
    {
        if config.late_evaluation() {
            if let Some(late_configs) = &mut self.late_configs {
                let typ = TypeId::of::<C>();
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

    pub fn on_update<T, H>(mut self, mut handler: H) -> Self
    where
        H: Handler<S, T> + 'static,
    {
        self.update_handler = Box::new(move |storage| handler.call(storage));
        self
    }

    pub fn on_close<T, H>(mut self, mut handler: H) -> Self
    where
        H: Handler<S, T> + 'static,
    {
        self.close_handler = Box::new(move |storage| handler.call(storage));
        self
    }

    pub fn listen_event<E, T, H>(mut self, mut handler: H) -> Self
    where
        E: 'static,
        H: EventHandler<E, S, T> + 'static,
    {
        let k = TypeId::of::<E>();
        let cb: Box<EventHandlerFn<E, S>> =
            Box::new(move |s: &mut Storage<S>, e: &E| handler.call(s, e));
        self.event_handler.entry(k).or_default().push(Box::new(cb));
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

    pub fn add_plugin_with<T, P, H>(mut self, handler: H) -> Result<Self, String>
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
            for (_, mut config) in late_configs {
                self = config.apply(self)?;
            }
        }

        let Self {
            mut plugins,
            mut runner,
            setup_handler,
            update_handler,
            event_handler,
            close_handler,
            init_handler,
            ..
        } = self;

        let state = (setup_handler)(&mut plugins)?;
        let storage = Storage {
            plugins,
            state,
            events: EventQueue::new(),
        };

        let app = App {
            storage,
            init_handler,
            event_handler,
            update_handler,
            close_handler,
            initialized: false,
            closed: false,
        };

        (runner)(app)?;

        Ok(())
    }
}
