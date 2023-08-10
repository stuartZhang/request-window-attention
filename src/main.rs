// https://stackoverflow.com/q/68313104/7202630
use ::std::{ffi::OsStr, iter, mem, os::windows::ffi::OsStrExt};
use ::winapi::{_core::ptr::null_mut, um::winuser::{FindWindowW, FlashWindowEx, FLASHWINFO, FLASHW_ALL}};
fn main() {
    let winname = win32_string("敏行");
    let hwnd = unsafe {
        FindWindowW(null_mut(),winname.as_ptr())
    };
    if hwnd.is_null() {
        return;
    }
    #[cfg(debug_assertions)]
    println!("HWND: {:?}", hwnd);
    let mut flash_info = FLASHWINFO {
        cbSize: mem::size_of::<FLASHWINFO>() as u32,
        hwnd,
        dwFlags: FLASHW_ALL,
        uCount: 10_u32,
        dwTimeout: 500_u32
    };
    let result = unsafe {
        FlashWindowEx(&mut flash_info)
    };
    #[cfg(debug_assertions)]
    println!("result: {:?}", result);
}
fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(iter::once(0)).collect()
}
