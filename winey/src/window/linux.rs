use crate::window::WindowInitialization;
use crate::window::{ControlFlow, Flow};
use crate::{Cursor, WindowEvent, WindowLevel, WindowRect, WindowType, WineyWindowImplementation};
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle, XlibDisplayHandle,
    XlibWindowHandle,
};
use safex::xlib::{AsRaw, Color, ColorMap, Window};
use std::ffi::c_void;

pub struct _Window {
    window: safex::xlib::Window,
    screen: safex::xlib::Screen,
    display: safex::xlib::Display,
}

impl _Window {
    pub(crate) fn run<C: FnMut(WindowEvent, &mut ControlFlow)>(&self, mut callback: C) {
        let mut control_flow = ControlFlow::new(Flow::Listen);
        self.window.run(|event, c| {
            callback(crate::WindowEvent::Update, &mut control_flow);
            match event {
                safex::xlib::WindowEvent::Expose => {
                    callback(crate::WindowEvent::RedrawRequested, &mut control_flow);
                }
            }
        })
    }
}

impl WindowInitialization for _Window {
    fn new(title: &str, width: u32, height: u32) -> Self {
        let display = safex::xlib::Display::open(None);
        let screen = safex::xlib::Screen::default(&display);

        let root = Window::root_window(&display, &screen);

        let cmap = ColorMap::default(&display, &screen);

        let white = Color::from_rgb(&display, &cmap, 65535, 65535, 65535).get_pixel();

        let window = Window::create_simple(
            &display,
            &screen,
            Some(()),
            Some(root),
            0,
            0,
            width,
            height,
            1,
            0,
            white,
        );

        window.map();
        window.set_window_title(title);

        Self {
            window,
            screen,
            display,
        }
    }
}

impl WineyWindowImplementation for _Window {
    fn show(&self) {
        self.window.map();
    }

    fn hide(&self) {
        self.window.unmap();
    }

    fn set_maximize(&self, maximize: bool) {
        match maximize {
            true => self.window.map(),
            false => {
                self.show();
            }
        }
    }

    fn set_minimize(&self, minimize: bool) {
        match minimize {
            true => self.window.unmap(),
            false => self.window.map(),
        }
    }

    fn set_title(&self, title: &str) {
        self.window.set_window_title(title);
    }

    fn set_undecorated(&self, undecorated: bool) {}

    fn set_window_level(&self, level: WindowLevel) {
        todo!()
    }

    fn set_window_type(&self, type_: WindowType) {
        todo!()
    }

    fn set_cursor(&self, cursor: Cursor) {
        todo!()
    }

    fn get_title(&self) -> String {
        todo!()
    }

    fn get_window_pos(&self) -> (u32, u32) {
        todo!()
    }

    fn get_window_rect(&self) -> WindowRect {
        todo!()
    }

    fn get_current_cursor(&self) -> Cursor {
        todo!()
    }
}

unsafe impl HasRawWindowHandle for _Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = XlibWindowHandle::empty();
        handle.window = self.window.as_raw();
        RawWindowHandle::Xlib(handle)
    }
}

unsafe impl HasRawDisplayHandle for _Window {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        let mut handle = XlibDisplayHandle::empty();
        handle.screen = self.screen.as_raw();
        handle.display = self.display.as_raw() as *mut c_void;
        RawDisplayHandle::Xlib(handle)
    }
}
