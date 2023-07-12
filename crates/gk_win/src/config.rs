use crate::WindowManager;
use gk_app::window::{GKWindow, GKWindowManager};
use gk_app::{App, AppBuilder, BuildConfig, GKState};
use std::marker::PhantomData;

// TODO config only for gk_winit or gk_web

pub struct WindowsConfig<W, M, S, F>
where
    W: GKWindow,
    M: GKWindowManager<W>,
    S: GKState + 'static,
    F: FnMut(App<S>) -> Result<(), String> + 'static,
{
    backend: Option<(M, F)>,
    _w: PhantomData<W>,
    _s: PhantomData<S>,
}

impl<W, M, S, F> WindowsConfig<W, M, S, F>
where
    W: GKWindow,
    M: GKWindowManager<W>,
    S: GKState + 'static,
    F: FnMut(App<S>) -> Result<(), String> + 'static,
{
    pub fn with_manager(manager: M, runner: F) -> Self {
        Self {
            backend: Some((manager, runner)),
            _w: Default::default(),
            _s: Default::default(),
        }
    }
}

impl<W, M, S, F> BuildConfig<S> for WindowsConfig<W, M, S, F>
where
    W: GKWindow + 'static,
    M: GKWindowManager<W> + 'static,
    S: GKState + 'static,
    F: FnMut(App<S>) -> Result<(), String> + 'static,
{
    fn apply(&mut self, builder: AppBuilder<S>) -> Result<AppBuilder<S>, String> {
        let (manager, runner) = self
            .backend
            .take()
            .ok_or_else(|| "Windows Manager needs an inner backend".to_string())?;
        Ok(builder
            .add_plugin(WindowManager::new(manager))
            .with_runner(runner))
    }
}
