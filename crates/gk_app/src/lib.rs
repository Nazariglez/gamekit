use gk_core::{GKWindow, GKWindowId, GKWindowManager};
use gk_winit::{EventLoopWindowTarget, Manager, Window};
use std::marker::PhantomData;
use std::ops::Rem;

pub struct Builder<W: GKWindow, WM: GKWindowManager<W>> {
    _w: PhantomData<W>,
    manager: WM,
}

impl<W: GKWindow, WM: GKWindowManager<W> + 'static> Builder<W, WM> {
    pub fn new() -> Result<Self, String> {
        let mut manager = WM::new()?;
        // let id = manager.create()?;
        Ok(Self {
            manager,
            _w: PhantomData::default(),
        })
    }

    pub fn run(mut self) {
        let Self { manager, .. } = self;
        let mut count = 0;
        let runner = manager.create_runner(move |mut manager| {
            if count < 3000 && count.rem(1000) == 0 {
                // manager.create();
            }

            count += 1;
        });
        // self.manager.run(|| {});
        runner();
    }
}

pub fn init() -> Result<Builder<Window, Manager>, String> {
    Builder::new()
}
