use crate::keyboard::VirtualKeyCode;

pub mod keyboard;
pub mod platform;
pub mod window;

pub struct WindowRect {
    pub bottom: i32,
    pub top: i32,
    pub left: i32,
    pub right: i32,
}

pub enum WindowEvent {
    /// Occurs at every frame.
    Update,
    KeyEvent(VirtualKeyCode),
    /// Occurs when the window is asked to redraw.
    RedrawRequested,
    /// Occurs when a window is about to be closed.
    CloseRequested,
}

pub enum WindowLevel {
    Normal,
    TopLevel,
}

pub enum WindowType {
    Normal,
    Utility,
}

#[derive(Clone, Copy, Debug)]
pub enum CursorIcon {
    Arrow,
    Hand,
    Help,
    Wait,
}

#[derive(Clone, Copy, Debug)]
pub struct Cursor {
    pub icon: CursorIcon,
    pub x: u32,
    pub y: u32,
}

pub trait WineyWindowImplementation {
    /// Display window.
    #[deprecated(since = "0.1.2", note = "Use set_visible to show or hide the window")]
    fn show(&self);
    /// Hide window.
    #[deprecated(since = "0.1.2", note = "Use set_visible to show or hide the window")]
    fn hide(&self);
    // Setters
    /// Maximize the window.
    fn set_maximize(&self, maximize: bool);
    /// Minimize the window
    fn set_minimize(&self, minimize: bool);
    /// Set window title
    fn set_title(&self, title: &str);
    /// Set window undecorated.
    fn set_undecorated(&self, undecorated: bool);
    fn set_window_level(&self, level: WindowLevel);
    fn set_window_type(&self, type_: WindowType);
    fn set_cursor(&self, cursor: Cursor);
    fn set_transparent(&self, transparent: bool);
    fn set_visible(&self, visible: bool);
    fn set_resizable(&self, resizable: bool);


    // Getters
    fn title(&self) -> String;
    fn position(&self) -> (u32, u32);
    fn rect(&self) -> WindowRect;
    fn current_cursor(&self) -> Cursor;
}
