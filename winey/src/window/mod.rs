use std::cmp::max;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle};
use crate::{WindowEvent, WineyWindowImplementation};

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

pub(crate) trait WindowInitialization {
    fn new(title: &str,width:u32,height:u32) -> Self;
}

pub struct Window {
    inner: _Window
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            inner: _Window::new(title,width,height)
        }
    }

    pub fn run<C: FnMut(WindowEvent)>(&self, mut callback: C) {
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
}

#[cfg(target_os = "windows")]
impl crate::platform::WindowExtForWindows for Window {
    fn set_window_corner_radius(&self, corner: crate::platform::WindowCorner) {
        self.inner.set_window_corner_radius(corner);
    }
}