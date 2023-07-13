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

#[derive(Debug, Clone)]
pub struct GKWindowAttributes {
    pub size: Option<(u32, u32)>,
    pub min_size: Option<(u32, u32)>,
    pub max_size: Option<(u32, u32)>,
    pub position: Option<(i32, i32)>,
    pub resizable: bool,
    pub title: String,
    pub fullscreen: bool,
    pub maximized: bool,
    pub visible: bool,
    pub transparent: bool,
}

impl Default for GKWindowAttributes {
    fn default() -> Self {
        Self {
            size: Some((800, 600)),
            min_size: None,
            max_size: None,
            position: None,
            resizable: false,
            title: "GameKit Window".to_string(),
            fullscreen: false,
            maximized: false,
            visible: true,
            transparent: false,
        }
    }
}

pub trait GKWindowManager<W: GKWindow> {
    fn create(&mut self, attrs: GKWindowAttributes) -> Result<GKWindowId, String>;
    fn window(&mut self, id: GKWindowId) -> Option<&mut W>;
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
    fn title(&self) -> &str;
    fn set_title(&mut self, title: &str);
    fn fullscreen(&self) -> bool;
    fn set_fullscreen(&mut self, fullscreen: bool);
    fn request_focus(&mut self);
    fn has_focus(&self) -> bool;
    fn set_cursor_icon(&mut self, cursor: CursorIcon);
    fn cursor(&self) -> CursorIcon;
    fn set_maximized(&mut self, maximized: bool);
    fn maximized(&self) -> bool;
    fn set_minimized(&mut self, minimized: bool);
    fn minimized(&self) -> bool;
    fn set_visible(&mut self, visible: bool);
    fn visible(&self) -> bool;
    fn set_transparent(&mut self, transparent: bool);
    fn transparent(&self) -> bool;
    fn set_resizable(&mut self, resizable: bool);
    fn resizable(&self) -> bool;
    fn set_min_size(&mut self, width: u32, height: u32);
    fn min_size(&self) -> Option<(u32, u32)>;
    fn set_max_size(&mut self, width: u32, height: u32);
    fn max_size(&self) -> Option<(u32, u32)>;
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

/// Represent mouse cursor icon
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub enum CursorIcon {
    Default,
    None,
    ContextMenu,
    Help,
    PointingHand,
    Progress,
    Wait,
    Cell,
    Crosshair,
    Text,
    VerticalText,
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    AllScroll,
    ResizeHorizontal,
    ResizeNeSw,
    ResizeNwSe,
    ResizeVertical,
    ZoomIn,
    ZoomOut,
    ResizeEast,
    ResizeSouthEast,
    ResizeSouth,
    ResizeSouthWest,
    ResizeWest,
    ResizeNorthWest,
    ResizeNorth,
    ResizeNorthEast,
    ResizeColumn,
    ResizeRow,
}