use std::ffi::{c_int, c_void, OsStr};
use std::mem::size_of;
use std::os::windows::ffi::OsStrExt;
use std::ptr::{null, null_mut};
use raw_window_handle::{HasRawWindowHandle, RawDisplayHandle, RawWindowHandle, Win32WindowHandle};
use winapi::ENUM;
use winapi::shared::minwindef::*;
use winapi::shared::windef::HWND;
use winapi::um::dwmapi::DwmSetWindowAttribute;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::*;
use crate::platform::{WindowCorner, WindowExtForWindows};
use crate::window::WindowInitialization;
use crate::WineyWindowImplementation;

ENUM! {enum DWMWINDOWATTRIBUTE {
      DWMWA_WINDOW_CORNER_PREFERENCE = 33,
}}

ENUM! {enum DWM_WINDOW_CORNER_PREFERENCE {
        DWMWCP_DEFAULT      = 0,
        DWMWCP_DONOTROUND   = 1,
        DWMWCP_ROUND        = 2,
        DWMWCP_ROUNDSMALL   = 3,
}}

pub struct _Window {
    hinstance: HMODULE,
    hwnd: HWND
}

impl _Window {

}

impl WindowInitialization for _Window {
    fn new(title: &str,width:u32,height:u32) -> Self {
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

            let wc = WNDCLASSW {
                hCursor: std::ptr::null_mut(),
                hInstance: hinstance,
                lpszClassName: window_class.as_ptr(),
                style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
                lpfnWndProc: Some(wndproc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hIcon: std::ptr::null_mut(),
                hbrBackground: std::ptr::null_mut(),
                lpszMenuName: std::ptr::null(),
            };

            RegisterClassW(&wc);

            let mut msg = 0;

            let hwnd = CreateWindowExW(
                0,
                window_class.as_ptr(),
                title_wide.as_ptr(),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width as c_int,
                height as c_int,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                hinstance,
                &mut msg as *mut i32 as _,
            );

            Self {
                hinstance,
                hwnd,
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
                    (WS_POPUP | WS_BORDER) as winapi::shared::ntdef::LONG,
                );
                SetWindowPos(
                    self.hwnd,
                    null_mut(),
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
                    WS_OVERLAPPEDWINDOW as winapi::shared::ntdef::LONG,
                );
            },
        }
    }

    fn run<C: FnMut()>(&self, callback: C) {
        unsafe {
            let mut message = std::mem::zeroed();

            while GetMessageW(&mut message, null_mut(), 0, 0) != 0 {
                DispatchMessageW(&message);
            }
        }
    }
}

impl WindowExtForWindows for _Window {
    fn set_window_corner_radius(&self, corner: WindowCorner) {
        unsafe {
            match corner {
                WindowCorner::DoNotRound => {
                    DwmSetWindowAttribute(
                        self.hwnd,
                        DWMWA_WINDOW_CORNER_PREFERENCE,
                        &DWMWCP_DONOTROUND as *const u32 as *const c_void as LPCVOID,
                        size_of::<u32>() as DWORD,
                    );
                }
                WindowCorner::SmallRound => {
                    DwmSetWindowAttribute(
                        self.hwnd,
                        DWMWA_WINDOW_CORNER_PREFERENCE,
                        &DWMWCP_ROUNDSMALL as *const u32 as *const c_void as LPCVOID,
                        size_of::<u32>() as DWORD,
                    );
                }
                WindowCorner::Round => {
                    DwmSetWindowAttribute(
                        self.hwnd,
                        DWMWA_WINDOW_CORNER_PREFERENCE,
                        &DWMWCP_ROUND as *const u32 as *const c_void as LPCVOID,
                        size_of::<u32>() as DWORD,
                    );
                }
            }
        }
    }
}

extern "system" fn wndproc(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    unsafe {
        match Msg {
            _ => DefWindowProcW(hWnd, Msg, wParam, lParam),
        }
    }
}