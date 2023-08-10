// https://stackoverflow.com/q/68313104/7202630
use ::std::{ffi::OsStr, iter, os::windows::ffi::OsStrExt};
use ::winapi::{_core::ptr::null_mut, shared::{minwindef::DWORD, windef::HWND}, um::winuser::{FindWindowW, FlashWindow}};
fn main() {
    let winname: Vec<u16> = win32_string("敏行");
    let hwnd: HWND = unsafe {
        FindWindowW(null_mut(),winname.as_ptr())
    };
    if !hwnd.is_null() {
        println!("HWND: {:?}", hwnd);
        unsafe {
            FlashWindow(hwnd, 0)
        };
    }
}
fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(iter::once(0)).collect()
}
