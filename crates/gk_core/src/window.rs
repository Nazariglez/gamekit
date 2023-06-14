#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
pub struct GKWindowId(u64);

impl GKWindowId {
    pub fn new(inner: u64) -> Self {
        Self(inner)
    }
}

pub trait GKWindowManager<T: GKWindow> {
    fn new() -> Result<Self, String> where Self: Sized;
    fn create(&mut self) -> Result<GKWindowId, String>;
    fn window(&mut self, id: GKWindowId) -> Option<&mut T>;
    fn close(&mut self, id: GKWindowId) -> bool;
    fn run<F: FnMut()>(&mut self, f: F);
    fn create_runner<F: FnMut(&mut Self) + 'static>(self, f: F) -> Box<dyn FnOnce()>;
}

pub trait GKWindow {
    fn id(&self) -> GKWindowId;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}