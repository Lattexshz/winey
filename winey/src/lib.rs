pub mod window;
pub mod platform;

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

pub trait WineyWindowImplementation {
    /// Display window.
    fn show(&self);
    /// Hide window.
    fn hide(&self);
    /// Maximize the window.
    fn set_maximize(&self,maximize: bool);
    /// Minimize the window
    fn set_minimize(&self,minimize: bool);
    /// Set window title
    fn set_title(&self,title: &str);
    /// Set window undecorated.
    fn set_undecorated(&self,undecorated: bool);
}