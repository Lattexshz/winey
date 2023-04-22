use crate::keyboard::{KeyState, VirtualKeyCode};

pub(crate) mod vk {
    use std::ffi::{c_int};
    pub const KEY_A: c_int = 0;
    pub const KEY_B: c_int = 1;
    pub const KEY_C: c_int = 2;
    pub const KEY_D: c_int = 3;
    pub const KEY_E: c_int = 4;
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

    pub const KEY_BACKSPACE: c_int = 0;
    pub const KEY_TAB: c_int = 0;
    pub const KEY_SHIFT: c_int = 0;
}

pub fn _get_key_name(code: VirtualKeyCode) -> String {
    "".to_string()
}

pub fn _get_key_state(code: VirtualKeyCode) -> KeyState {
    KeyState::None
}
