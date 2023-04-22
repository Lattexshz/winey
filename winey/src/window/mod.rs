use crate::{Cursor, CursorIcon, WindowEvent, WindowLevel, WindowRect, WindowType, WineyWindowImplementation};
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};


#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use self::linux::*;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use self::macos::*;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::*;

pub enum Flow {
    Listen,
    Exit(i32),
}

pub struct ControlFlow {
    flow: Flow,
}

impl ControlFlow {
    pub fn new(flow: Flow) -> Self {
        Self { flow }
    }

    pub fn listen(&mut self) {
        self.flow = Flow::Listen
    }

    pub fn exit(&mut self, code: i32) {
        self.flow = Flow::Exit(code);
    }
}

pub(crate) trait WindowInitialization {
    fn new(title: &str, width: u32, height: u32) -> Self;
}

pub struct Window {
    inner: _Window,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            inner: _Window::new(title, width, height),
        }
    }

    pub fn run<C: FnMut(WindowEvent, &mut ControlFlow)>(&self, callback: C) {
        self.inner.run(callback);
    }
}

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.inner.raw_window_handle()
    }
}

unsafe impl HasRawDisplayHandle for Window {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        self.inner.raw_display_handle()
    }
}

impl WineyWindowImplementation for Window {
    fn show(&self) {
        self.inner.show();
    }

    fn hide(&self) {
        self.inner.hide()
    }

    fn set_maximize(&self, maximize: bool) {
        self.inner.set_maximize(maximize);
    }

    fn set_minimize(&self, minimize: bool) {
        self.inner.set_minimize(minimize);
    }

    fn set_title(&self, title: &str) {
        self.inner.set_title(title);
    }

    fn set_undecorated(&self, undecorated: bool) {
        self.inner.set_undecorated(undecorated);
    }

    fn set_window_level(&self, level: WindowLevel) {
        self.inner.set_window_level(level);
    }

    fn set_window_type(&self, type_: WindowType) {
        self.inner.set_window_type(type_)
    }

    fn set_cursor(&self, icon: Cursor) {
        self.inner.set_cursor(icon);
    }

    fn get_title(&self) -> String {
        self.inner.get_title()
    }

    fn get_window_pos(&self) -> (u32, u32) {
        self.inner.get_window_pos()
    }

    fn get_window_rect(&self) -> WindowRect {
        self.inner.get_window_rect()
    }

    fn get_current_cursor(&self) -> Cursor {
        self.inner.get_current_cursor()
    }
}

#[cfg(target_os = "windows")]
impl crate::platform::WindowExtForWindows for Window {
    fn set_window_corner_radius(&self, corner: crate::platform::WindowCorner) {
        self.inner.set_window_corner_radius(corner);
    }

    fn set_window_border_color(&self, r: u8, g: u8, b: u8) {
        self.inner.set_window_border_color(r, g, b);
    }

    fn set_window_caption_color(&self, r: u8, g: u8, b: u8) {
        self.inner.set_window_caption_color(r, g, b);
    }

    fn set_window_text_color(&self, r: u8, g: u8, b: u8) {
        self.inner.set_window_text_color(r, g, b);
    }

    fn extend_frame_into_client_area(&self, rect: crate::platform::windows::Margin) {
        self.inner.extend_frame_into_client_area(rect);
    }
}
