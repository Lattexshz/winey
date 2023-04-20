use crate::platform::{Margin, WindowCorner, WindowExtForWindows};
use crate::window::{ControlFlow, Flow, WindowInitialization};
use crate::{
    CursorIcon, KeyCode, WindowEvent, WindowLevel, WindowRect, WindowType,
    WineyWindowImplementation,
};
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle, Win32WindowHandle,
    WindowsDisplayHandle,
};

use std::ffi::{c_uchar, c_void, OsStr};
use std::mem::size_of;
use std::os::windows::ffi::OsStrExt;
use std::ptr::{addr_of, addr_of_mut};


use windows_sys::core::{PSTR};
use windows_sys::Win32::Foundation::{COLORREF, HMODULE, HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows_sys::Win32::Graphics::Dwm::*;

use windows_sys::Win32::System::LibraryLoader::*;
use windows_sys::Win32::UI::Controls::MARGINS;
use windows_sys::Win32::UI::WindowsAndMessaging::*;


pub struct _Window {
    hinstance: HMODULE,
    hwnd: HWND,
}

impl _Window {}

impl _Window {
    pub(crate) fn run<C: FnMut(WindowEvent, &mut ControlFlow)>(&self, mut callback: C) {
        let mut message = unsafe { core::mem::zeroed() };
        let mut control_flow = ControlFlow::new(Flow::Listen);

        unsafe {
            match control_flow.flow {
                Flow::Listen => {
                    while GetMessageW(&mut message, 0, 0, 0) != 0 {
                        callback(WindowEvent::Update, &mut control_flow);
                        TranslateMessage(&mut message);
                        DispatchMessageW(&message);
                        callback(WindowEvent::Update, &mut control_flow);
                        match message.message {
                            WM_PAINT => {
                                callback(WindowEvent::RedrawRequested, &mut control_flow);
                            }
                            WM_KEYUP => callback(
                                WindowEvent::KeyUp(KeyCode(message.wParam as u32)),
                                &mut control_flow,
                            ),
                            WM_KEYDOWN => {
                                callback(
                                    WindowEvent::KeyDown(KeyCode(message.wParam as u32)),
                                    &mut control_flow,
                                );
                            }
                            _ => {}
                        }

                        match MSG.message {
                            WM_CLOSE => {
                                callback(WindowEvent::CloseRequested, &mut control_flow);
                            }
                            _ => {}
                        }
                    };
                }

                Flow::Exit(code) => {
                    PostQuitMessage(code);
                }
            }
        }
    }
}

impl WindowInitialization for _Window {
    fn new(title: &str, _width: u32, _height: u32) -> Self {
        unsafe {
            SetProcessDPIAware();
        }
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
                style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
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
            Self { hinstance, hwnd }
        }
    }
}

impl WineyWindowImplementation for _Window {
    fn show(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_SHOW);
        }
    }

    fn hide(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_HIDE);
        }
    }

    fn set_maximize(&self, maximize: bool) {
        match maximize {
            true => unsafe {
                ShowWindow(self.hwnd, SW_MAXIMIZE);
            },
            false => {
                self.show();
            }
        }
    }

    fn set_minimize(&self, minimize: bool) {
        match minimize {
            true => unsafe {
                ShowWindow(self.hwnd, SW_MINIMIZE);
            },
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
            SetWindowTextW(self.hwnd, title_wide.as_ptr());
        }
    }

    fn set_undecorated(&self, undecorated: bool) {
        match undecorated {
            true => unsafe {
                self.set_window_corner_radius(WindowCorner::Round);
                SetWindowLongW(self.hwnd, GWL_STYLE, (WS_POPUP | WS_BORDER) as i32);
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
                SetWindowLongW(self.hwnd, GWL_STYLE, WS_OVERLAPPEDWINDOW as i32);
            },
        }
    }

    fn set_window_level(&self, level: WindowLevel) {
        match level {
            WindowLevel::Normal => unsafe {
                SetWindowPos(self.hwnd, HWND_DESKTOP, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
            },

            WindowLevel::TopLevel => unsafe {
                SetWindowPos(self.hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
            },
        }
    }

    fn set_window_type(&self, type_: WindowType) {
        match type_ {
            WindowType::Normal => unsafe {
                SetWindowLongW(self.hwnd, GWL_EXSTYLE, 0);
            },
            WindowType::Utility => unsafe {
                SetWindowLongW(self.hwnd, GWL_EXSTYLE, WS_EX_TOOLWINDOW as i32);
            },
        }
    }

    fn set_cursor_icon(&self, icon: CursorIcon) {
        unsafe {
            let icon = match icon {
                CursorIcon::Arrow => LoadCursorW(0, IDC_ARROW),
                CursorIcon::Hand => LoadCursorW(0, IDC_HAND),
                CursorIcon::Help => LoadCursorW(0, IDC_HELP),
                CursorIcon::Wait => LoadCursorW(0, IDC_WAIT),
            };

            SetClassLongPtrA(self.hwnd, GCLP_HCURSOR, icon as isize);
        }
    }

    fn get_title(&self) -> String {
        let mut buffer = [0u8; 1024];

        unsafe {
            GetWindowTextA(self.hwnd, addr_of_mut!(buffer) as PSTR, 1024);
            let mut del = 0;
            for i in buffer {
                if i == 0 {
                    del = i;
                    break;
                }
            }
            let buffer = std::str::from_utf8(&buffer).unwrap().to_owned();
            let buffer = buffer.trim_matches(char::from(0));
            buffer.to_owned()
        }
    }

    fn get_window_pos(&self) -> (u32, u32) {
        unsafe {
            let mut rect = std::mem::zeroed();
            GetWindowRect(self.hwnd, &mut rect);
            (rect.left.try_into().unwrap(), rect.top.try_into().unwrap())
        }
    }

    fn get_window_rect(&self) -> WindowRect {
        unsafe {
            let rect = std::mem::zeroed();
            GetWindowRect(self.hwnd, rect);

            WindowRect {
                bottom: (*rect).bottom,
                top: (*rect).top,
                left: (*rect).left,
                right: (*rect).right,
            }
        }
    }
}

impl WindowExtForWindows for _Window {
    fn set_window_corner_radius(&self, corner: WindowCorner) {
        unsafe {
            match corner {
                WindowCorner::DoNotRound => {
                    let _a = DWMWCP_DONOTROUND;
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

    fn set_window_border_color(&self, r: u8, g: u8, b: u8) {
        unsafe {
            DwmSetWindowAttribute(
                self.hwnd,
                DWMWA_BORDER_COLOR,
                &RGB(r, g, b) as *const u32 as *const c_void,
                size_of::<u32>() as u32,
            );
        }
    }

    fn set_window_caption_color(&self, r: u8, g: u8, b: u8) {
        unsafe {
            DwmSetWindowAttribute(
                self.hwnd,
                DWMWA_CAPTION_COLOR,
                &RGB(r, g, b) as *const u32 as *const c_void,
                size_of::<u32>() as u32,
            );
        }
    }

    fn set_window_text_color(&self, r: u8, g: u8, b: u8) {
        unsafe {
            DwmSetWindowAttribute(
                self.hwnd,
                DWMWA_TEXT_COLOR,
                &RGB(r, g, b) as *const u32 as *const c_void,
                size_of::<u32>() as u32,
            );
        }
    }

    fn extend_frame_into_client_area(&self, rect: Margin) {
        unsafe {
            let margins = MARGINS {
                cxLeftWidth: rect.left_width,
                cxRightWidth: rect.right_width,
                cyTopHeight: rect.top_height,
                cyBottomHeight: rect.bottom_height,
            };

            DwmExtendFrameIntoClientArea(self.hwnd, addr_of!(margins));
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
        let handle = WindowsDisplayHandle::empty();
        RawDisplayHandle::Windows(handle)
    }
}

static mut MSG: MSG = MSG {
    hwnd: 0,
    message: 0,
    wParam: 0,
    lParam: 0,
    time: 0,
    pt: POINT { x: 0, y: 0 },
};

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
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
    }
}

fn RGB(r: c_uchar, g: c_uchar, b: c_uchar) -> COLORREF {
    (r as COLORREF | ((g as COLORREF) << 8) | ((b as COLORREF) << 16)) as COLORREF
}
