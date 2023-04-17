pub mod window;
pub mod platform;

pub struct WindowRect {
    pub bottom: i32,
    pub top: i32,
    pub left: i32,
    pub right: i32
}

#[derive(Clone,Copy,Debug,PartialEq)]
#[repr(C)]
pub struct KeyCode(pub(crate) u32);

impl Into<char> for KeyCode {
    fn into(self) -> char {
        char::from_u32(self.0).unwrap()
    }
}

impl Into<u32> for KeyCode {
    fn into(self) -> u32 {
        self.0
    }
}

pub enum WindowEvent {
    /// Occurs at every frame.
    Update,
    KeyDown(KeyCode),
    KeyUp(KeyCode),
    /// Occurs when the window is asked to redraw.
    RedrawRequested,
    /// Occurs when a window is about to be closed.
    CloseRequested
}

pub enum WindowLevel {
    Normal
}

pub enum WindowType {
    Utility
}

pub enum CursorIcon {
    Arrow
}

pub trait WineyWindowImplementation {
    /// Display window.
    fn show(&self);
    /// Hide window.
    fn hide(&self);
    // Setters
    /// Maximize the window.
    fn set_maximize(&self,maximize: bool);
    /// Minimize the window
    fn set_minimize(&self,minimize: bool);
    /// Set window title
    fn set_title(&self,title: &str);
    /// Set window undecorated.
    fn set_undecorated(&self,undecorated: bool);
    fn set_fullscreen(&self,fullscreen: bool);
    fn set_window_level(&self,level: WindowLevel);
    fn set_window_type(&self,type_: WindowType);
    fn set_cursor_icon(&self,icon: CurosrIcon);

    // Getters
    fn get_title(&self) -> String;
    fn get_window_pos(&self) -> (u32,u32);
    fn get_window_rect(&self) -> WindowRect;
}