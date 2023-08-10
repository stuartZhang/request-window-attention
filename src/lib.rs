#![cfg_attr(debug_assertions, feature(trace_macros, log_syntax))]
// https://stackoverflow.com/q/68313104/7202630
// https://github.com/infinyon/node-bindgen#readme
use ::std::{ffi::{c_char, c_uint, CString, OsString}, iter, mem, os::windows::ffi::OsStrExt};
use ::winapi::{_core::ptr::null_mut, um::winuser::{FindWindowW, FlashWindowEx, FLASHWINFO, FLASHW_ALL}};

pub fn flash(process_name: OsString, count: u32, blink_rate: u32){
    let winname = process_name.encode_wide().chain(iter::once(0)).collect::<Vec<u16>>();
    let hwnd = unsafe { FindWindowW(null_mut(),winname.as_ptr()) };
    if hwnd.is_null() {
        return;
    }
    #[cfg(debug_assertions)]
    println!("HWND: {:?}", hwnd);
    let mut flash_info = FLASHWINFO {
        cbSize: mem::size_of::<FLASHWINFO>() as u32,
        hwnd,
        dwFlags: FLASHW_ALL,
        uCount: count,
        dwTimeout: blink_rate
    };
    let result = unsafe { FlashWindowEx(&mut flash_info) };
    #[cfg(debug_assertions)]
    println!("result: {:?}", result);
}
pub extern fn flash_c(process_name: *const c_char, count: c_uint, blink_rate: c_uint) {
    let process_name = unsafe { CString::from_raw(process_name as *mut i8) }.into_string().unwrap();
    flash(process_name.into(), count, blink_rate)
}