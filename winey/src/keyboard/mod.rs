#[cfg(target_os = "linux")]
pub(crate) mod linux;
#[cfg(target_os = "linux")]
pub(crate) use self::linux::*;

#[cfg(target_os = "macos")]
pub(crate) mod macos;
#[cfg(target_os = "macos")]
pub(crate) use self::macos::*;

#[cfg(target_os = "windows")]
pub(crate) mod windows;
#[cfg(target_os = "windows")]
pub(crate) use self::windows::*;

pub type VirtualKeyCode = u32;

pub const KEY_A: VirtualKeyCode = vk::KEY_A as VirtualKeyCode;
pub const KEY_B: VirtualKeyCode = vk::KEY_B as VirtualKeyCode;
pub const KEY_C: VirtualKeyCode = vk::KEY_C as VirtualKeyCode;
pub const KEY_D: VirtualKeyCode = vk::KEY_D as VirtualKeyCode;
pub const KEY_E: VirtualKeyCode = vk::KEY_E as VirtualKeyCode;
pub const KEY_F: VirtualKeyCode = vk::KEY_F as VirtualKeyCode;
pub const KEY_G: VirtualKeyCode = vk::KEY_G as VirtualKeyCode;
pub const KEY_H: VirtualKeyCode = vk::KEY_H as VirtualKeyCode;
pub const KEY_I: VirtualKeyCode = vk::KEY_I as VirtualKeyCode;
pub const KEY_J: VirtualKeyCode = vk::KEY_J as VirtualKeyCode;
pub const KEY_K: VirtualKeyCode = vk::KEY_K as VirtualKeyCode;
pub const KEY_L: VirtualKeyCode = vk::KEY_L as VirtualKeyCode;
pub const KEY_M: VirtualKeyCode = vk::KEY_M as VirtualKeyCode;
pub const KEY_N: VirtualKeyCode = vk::KEY_N as VirtualKeyCode;
pub const KEY_O: VirtualKeyCode = vk::KEY_O as VirtualKeyCode;
pub const KEY_P: VirtualKeyCode = vk::KEY_P as VirtualKeyCode;
pub const KEY_Q: VirtualKeyCode = vk::KEY_Q as VirtualKeyCode;
pub const KEY_R: VirtualKeyCode = vk::KEY_R as VirtualKeyCode;
pub const KEY_S: VirtualKeyCode = vk::KEY_S as VirtualKeyCode;
pub const KEY_T: VirtualKeyCode = vk::KEY_T as VirtualKeyCode;
pub const KEY_U: VirtualKeyCode = vk::KEY_U as VirtualKeyCode;
pub const KEY_V: VirtualKeyCode = vk::KEY_V as VirtualKeyCode;
pub const KEY_W: VirtualKeyCode = vk::KEY_W as VirtualKeyCode;
pub const KEY_X: VirtualKeyCode = vk::KEY_X as VirtualKeyCode;
pub const KEY_Y: VirtualKeyCode = vk::KEY_Y as VirtualKeyCode;
pub const KEY_Z: VirtualKeyCode = vk::KEY_Z as VirtualKeyCode;

pub const KEY_BACKSPACE: VirtualKeyCode = vk::KEY_BACKSPACE as VirtualkeyCode;
pub const KEY_SHIFT:VirtualKeyCode = vk::KEY_SHIFT as VirtualKeyCode;
pub const KEY_TAB: VirtualKeyCode = vk::KEY_TAB as VirtualkeyCode;

pub fn get_key_name(code: VirtualKeyCode) -> String {
    _get_key_name(code)
}