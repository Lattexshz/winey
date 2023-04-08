use std::cell::{Ref, RefCell};
use std::ffi::{c_int, c_void, OsStr};
use std::mem::size_of;
use std::os::windows::ffi::OsStrExt;
use std::ptr::{addr_of, null, null_mut};
use std::sync::Mutex;
use std::time::Duration;
use once_cell::unsync::{Lazy, OnceCell};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle, Win32WindowHandle, WindowsDisplayHandle};
use windows_sys::{s, w};
use windows_sys::Win32::Foundation::{HMODULE, HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows_sys::Win32::Graphics::Dwm::*;
use windows_sys::Win32::Graphics::Gdi::ValidateRect;
use windows_sys::Win32::System::LibraryLoader::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;
use crate::platform::{WindowCorner, WindowExtForWindows};
use crate::window::WindowInitialization;
use crate::{WindowEvent, WineyWindowImplementation};

pub struct _Window {
    hinstance: HMODULE,
    hwnd: HWND
}

impl _Window {

}

impl _Window {
   pub(crate) fn run<C: FnMut(WindowEvent)>(&self, mut callback: C) {
        let mut message = unsafe { core::mem::zeroed() };

        unsafe {

            loop {
                GetMessageW(&mut message, 0, 0, 0);
                TranslateMessage(&mut message);
                DispatchMessageW(&message);
                callback(WindowEvent::Update);
                match message.message {
                    WM_PAINT => {
                        callback(WindowEvent::RedrawRequested);
                    }
                    _ => {}
                }

                match MSG.message {
                    WM_CLOSE => {
                        callback(WindowEvent::CloseRequested);
                    }
                    _ => {}
                }
            }
        }
    }
}

impl WindowInitialization for _Window {
    fn new(title: &str, width: u32, height: u32) -> Self {
        let title_wide: Vec<u16> = OsStr::new(&title)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();

        let window_class = OsStr::new("window")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<_>>();

        unsafe {
            let hinstance = GetModuleHandleW(std::ptr::null());
            debug_assert!(hinstance != 0);

            let wc = WNDCLASSW {
                hCursor: LoadCursorW(0, IDC_ARROW),
                hInstance: hinstance,
                lpszClassName: window_class.as_ptr(),
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wndproc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hIcon: 0,
                hbrBackground: 0,
                lpszMenuName: std::ptr::null(),
            };

            let atom = RegisterClassW(&wc);
            debug_assert!(atom != 0);

            let hwnd = CreateWindowExW(
                0,
                window_class.as_ptr(),
                title_wide.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                0,
                0,
                hinstance,
                std::ptr::null(),
            );
            Self {
                hinstance,
                hwnd,
            }
        }
    }
}


impl WineyWindowImplementation for _Window {
    fn show(&self) {
        unsafe {
            ShowWindow(self.hwnd,SW_SHOW);
        }
    }

    fn hide(&self) {
        unsafe {
            ShowWindow(self.hwnd,SW_HIDE);
        }
    }

    fn set_maximize(&self, maximize: bool) {
        match maximize {
            true => {
                unsafe {
                    ShowWindow(self.hwnd,SW_MAXIMIZE);
                }
            }
            false => {
                self.show();
            }
        }
    }

    fn set_minimize(&self, minimize: bool) {
        match minimize {
            true => {
                unsafe {
                    ShowWindow(self.hwnd,SW_MINIMIZE);
                }
            }
            false => {
                self.show();
            }
        }
    }

    fn set_title(&self, title: &str) {
        let title_wide: Vec<u16> = OsStr::new(&title)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();

        unsafe {
            SetWindowTextW(self.hwnd,title_wide.as_ptr());
        }
    }

    fn set_undecorated(&self, undecorated: bool) {
        match undecorated {
            true => unsafe {
                self.set_window_corner_radius(WindowCorner::Round);
                SetWindowLongW(
                    self.hwnd,
                    GWL_STYLE,
                    (WS_POPUP | WS_BORDER) as i32,
                );
                SetWindowPos(
                    self.hwnd,
                    0,
                    0,
                    0,
                    0,
                    0,
                    SWP_DRAWFRAME | SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_NOZORDER,
                );
            },
            false => unsafe {
                SetWindowLongW(
                    self.hwnd,
                    GWL_STYLE,
                    WS_OVERLAPPEDWINDOW as i32,
                );
            },
        }
    }
}

impl WindowExtForWindows for _Window {
    fn set_window_corner_radius(&self, corner: WindowCorner) {
        unsafe {
            match corner {
                WindowCorner::DoNotRound => {
                    let a = DWMWCP_DONOTROUND;
                    DwmSetWindowAttribute(
                        self.hwnd,
                        DWMWA_WINDOW_CORNER_PREFERENCE,
                        &DWMWCP_DONOTROUND as *const i32 as *const c_void,
                        size_of::<u32>() as u32,
                    );
                }
                WindowCorner::SmallRound => {
                    DwmSetWindowAttribute(
                        self.hwnd,
                        DWMWA_WINDOW_CORNER_PREFERENCE,
                        &DWMWCP_ROUNDSMALL as *const i32 as *const c_void,
                        size_of::<u32>() as u32,
                    );
                }
                WindowCorner::Round => {
                    DwmSetWindowAttribute(
                        self.hwnd,
                        DWMWA_WINDOW_CORNER_PREFERENCE,
                        &DWMWCP_ROUND as *const i32 as *const c_void,
                        size_of::<u32>() as u32,
                    );
                }
            }
        }
    }
}

unsafe impl HasRawWindowHandle for _Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = Win32WindowHandle::empty();
        handle.hwnd = self.hwnd as *mut c_void;
        handle.hinstance = self.hinstance as *mut c_void;
        RawWindowHandle::Win32(handle)
    }
}

unsafe impl HasRawDisplayHandle for _Window {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        let mut handle = WindowsDisplayHandle::empty();
        RawDisplayHandle::Windows(handle)
    }
}

static mut MSG:MSG = MSG {
    hwnd: 0,
    message: 0,
    wParam: 0,
    lParam: 0,
    time: 0,
    pt: POINT { x: 0, y: 0 },
};

// extern "system" fn wndproc(hWnd: HWND, Msg: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
//     unsafe {
//         match Msg {
//             WM_CREATE => {
//                 //set_msg(Msg, wParam, lParam);
//                 0
//             }
//             WM_PAINT => {
//                 //set_msg(Msg, wParam, lParam);
//                 0
//             }
//             WM_DESTROY => {
//                 //set_msg(Msg, wParam, lParam);
//                 0
//             }
//             _ => DefWindowProcW(hWnd, Msg, wParam, lParam),
//         }
//     }
// }

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_CLOSE => {
                MSG = MSG {
                    hwnd: window,
                    message,
                    wParam: wparam,
                    lParam: lparam,
                    time: 0,
                    pt: POINT { x: 0, y: 0 },
                };
                0
            }
            _ => {
                DefWindowProcW(window, message, wparam, lparam)
            },
        }
    }
}