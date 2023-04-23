use crate::keyboard::{KeyState, VirtualKeyCode};

use std::ffi::{c_char, CStr};

use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, GetKeyNameTextA, MapVirtualKeyA, MAPVK_VK_TO_VSC,
};

pub(crate) mod vk {
    use std::ffi::c_int;
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;

    pub const KEY_A: c_int = 0x41;
    pub const KEY_B: c_int = 0x42;
    pub const KEY_C: c_int = 0x43;
    pub const KEY_D: c_int = 0x44;
    pub const KEY_E: c_int = 0x45;
    pub const KEY_F: c_int = 0x46;
    pub const KEY_G: c_int = 0x47;
    pub const KEY_H: c_int = 0x48;
    pub const KEY_I: c_int = 0x49;
    pub const KEY_J: c_int = 0x4A;
    pub const KEY_K: c_int = 0x4B;
    pub const KEY_L: c_int = 0x4C;
    pub const KEY_M: c_int = 0x4D;
    pub const KEY_N: c_int = 0x4E;
    pub const KEY_O: c_int = 0x4F;
    pub const KEY_P: c_int = 0x50;
    pub const KEY_Q: c_int = 0x51;
    pub const KEY_R: c_int = 0x52;
    pub const KEY_S: c_int = 0x53;
    pub const KEY_T: c_int = 0x54;
    pub const KEY_U: c_int = 0x55;
    pub const KEY_V: c_int = 0x56;
    pub const KEY_W: c_int = 0x57;
    pub const KEY_X: c_int = 0x58;
    pub const KEY_Y: c_int = 0x59;
    pub const KEY_Z: c_int = 0x5A;

    pub const KEY_BACKSPACE: VIRTUAL_KEY = VK_BACK;
    pub const KEY_TAB: VIRTUAL_KEY = VK_TAB;
    pub const KEY_SHIFT: VIRTUAL_KEY = VK_SHIFT;
}

pub fn _get_key_name(code: VirtualKeyCode) -> String {
    unsafe {
        let mut name: [u8; 32] = Default::default();
        let ptr = name.as_mut_ptr();
        let code = MapVirtualKeyA(code, MAPVK_VK_TO_VSC);
        GetKeyNameTextA((code << 16) as i32, ptr, 32);
        let string = CStr::from_ptr(ptr as *const c_char)
            .to_str()
            .unwrap()
            .to_owned();
        string
    }
}

pub fn _get_key_state(code: VirtualKeyCode) -> KeyState {
    unsafe {
        let mut state = KeyState::None;
        let result = GetAsyncKeyState(code as i32) as u16;
        if result == 0 {
            state = KeyState::None;
        } else if (result & 0x01) != 0 {
            state = KeyState::Downed;
        } else if (result & 0xff00) != 0 {
            state = KeyState::Pressing;
        };
        state
    }
}
