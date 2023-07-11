use gk_app::window::{GKWindow, GKWindowManager};
use gk_app::Plugin;
use std::marker::PhantomData;

pub struct WindowManager<W, M>
where
    W: GKWindow,
    M: GKWindowManager<W>,
{
    manager: M,
    _w: PhantomData<W>,
}

impl<W, M> WindowManager<W, M>
where
    W: GKWindow,
    M: GKWindowManager<W>,
{
    pub fn new(manager: M) -> Self {
        Self {
            manager,
            _w: Default::default(),
        }
    }

    pub fn raw(&self) -> &M {
        &self.manager
    }

    pub fn raw_mut(&mut self) -> &mut M {
        &mut self.manager
    }
}

impl<W, M> Plugin for WindowManager<W, M>
where
    W: GKWindow,
    M: GKWindowManager<W>,
{
}
