use std::marker::PhantomData;
use gk_core::{GKWindow, GKWindowManager, GKRunner, GKWindowId};
use gk_winit::{EventLoopWindowTarget, Manager, Window};

pub struct App<EH, W: GKWindow, WM: GKWindowManager<W, EH>> {
    _w: PhantomData<W>,
    _eh: PhantomData<EH>,
    manager: WM,
}

impl<EH, W: GKWindow, WM: GKWindowManager<W, EH> + 'static> App<EH, W, WM> {
    pub fn new() -> Result<Self, String> {
        let mut manager = WM::new()?;
        // let id = manager.create()?;
        Ok(Self {
            manager,
            _w: PhantomData::default(),
            _eh: PhantomData::default(),
        })
    }

    pub fn run(mut self) {
        let runner = WM::create_runner(move |eh| {
            println!("here");
            if self.manager.window(GKWindowId::new(0)).is_none() {
                self.manager.create(eh);
            }

            if self.manager.window(GKWindowId::new(1)).is_none() {
                self.manager.create(eh);
            }
        });
        // self.manager.run(|| {});
        runner();
    }
}

pub fn init() -> Result<App<EventLoopWindowTarget<()>, Window, Manager>, String> {
    App::new()
}