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
    fn create(&mut self) -> Result<GKWindowId, String>;
    fn window(&mut self, id: GKWindowId) -> Option<&mut T>;
    fn close(&mut self, id: GKWindowId) -> bool;
    fn exit(&mut self);
}

pub trait GKWindow {
    fn id(&self) -> GKWindowId;
    fn size(&self) -> (u32, u32);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn set_size(&mut self, width: u32, height: u32);
    fn scale(&self) -> f64;
    fn position(&self) -> Result<(i32, i32), String>;
    fn set_position(&mut self, x: i32, y: i32);
}

/// Window's event
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WindowEvent {
    pub id: GKWindowId,
    pub event: WindowEventId,
}

/// Window's event type
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WindowEventId {
    /// A new window was created
    Open,

    /// Window's position after it was moved
    Moved { x: i32, y: i32 },

    /// Window's size after it was resized
    Resized { width: u32, height: u32 },

    /// The window was minimized
    Minimized,

    /// The window was maximized
    Maximized,

    /// The window did gain the focus
    FocusGained,

    /// The window did lost the focus
    FocusLost,

    /// The window has received the close signal
    CloseRequest,
}
