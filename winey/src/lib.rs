pub mod window;
pub mod platform;

pub enum WindowEvent {
    /// Occurs at every frame.
    Update,
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