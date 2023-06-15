#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
pub struct GKWindowId(u64);

impl From<u64> for GKWindowId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<GKWindowId> for u64 {
    fn from(value: GKWindowId) -> Self {
        value.0
    }
}

pub trait GKWindowManager<T: GKWindow> {
    fn new() -> Result<Self, String> where Self: Sized;
    fn create(&mut self) -> Result<GKWindowId, String>;
    fn window(&mut self, id: GKWindowId) -> Option<&mut T>;
    fn close(&mut self, id: GKWindowId) -> bool;
    fn exit(&mut self);
    fn create_runner<F: FnMut(&mut Self) + 'static>(self, f: F) -> Box<dyn FnOnce()>;
}

pub trait GKWindow {
    fn id(&self) -> GKWindowId;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}