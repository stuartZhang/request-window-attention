mod enum_windows;

use ::std::{ffi::OsString, iter, mem, os::windows::ffi::OsStrExt};
use ::winapi::{_core::ptr::null_mut, um::winuser::{FindWindowW, FlashWindowEx, FLASHWINFO, FLASHW_CAPTION, FLASHW_TIMER, FLASHW_TIMERNOFG, FLASHW_STOP, FLASHW_TRAY}, shared::windef::HWND};

pub fn stop_flash_by_title(win_title: OsString, cb: Option<&Box<dyn Fn(String)>>) {
    flash_by_title(win_title, FLASHW_STOP, 0, 0, cb)
}
pub fn start_flash_by_title(win_title: OsString, count: u32, blink_rate: u32, cb: Option<&Box<dyn Fn(String)>>) {
    flash_by_title(win_title, FLASHW_CAPTION | FLASHW_TIMER | FLASHW_TIMERNOFG | FLASHW_TRAY, count, blink_rate, cb)
}
fn flash_by_title(win_title: OsString, action: u32, count: u32, blink_rate: u32, cb: Option<&Box<dyn Fn(String)>>){
    let winname = win_title.encode_wide().chain(iter::once(0)).collect::<Vec<u16>>();
    let hwnd = unsafe { FindWindowW(null_mut(), winname.as_ptr()) };
    if hwnd.is_null() {
        return;
    }
    flash_only(hwnd, action, count, blink_rate, cb);
}
//
pub fn stop_flash_by_ppid(process_id: u32, cb: Option<&Box<dyn Fn(String)>>){
    flash_by_ppid(process_id, FLASHW_STOP, 0, 0, cb)
}
pub fn start_flash_by_ppid(process_id: u32, count: u32, blink_rate: u32, cb: Option<&Box<dyn Fn(String)>>) {
    flash_by_ppid(process_id, FLASHW_CAPTION | FLASHW_TIMER | FLASHW_TIMERNOFG | FLASHW_TRAY, count, blink_rate, cb)
}
fn flash_by_ppid(process_id: u32, action: u32, count: u32, blink_rate: u32, cb: Option<&Box<dyn Fn(String)>>) {
    if let Some(hwnd) = enum_windows::enumerate_windows(process_id, cb) {
        flash_only(hwnd, action, count, blink_rate, cb);
    }
}
fn flash_only(hwnd: HWND, action: u32, count: u32, blink_rate: u32, cb: Option<&Box<dyn Fn(String)>>) {
    #[cfg(debug_assertions)]
    dbg!(hwnd);
    cb.map(|log| log(format!("[flash_only]hwnd={hwnd:?}; action={action}; count={count}; blink_rate={blink_rate}")));
    let mut flash_info = FLASHWINFO {
        cbSize: mem::size_of::<FLASHWINFO>() as u32,
        hwnd,
        dwFlags: action,
        uCount: count,
        dwTimeout: blink_rate
    };
    let result = unsafe { FlashWindowEx(&mut flash_info) };
    #[cfg(debug_assertions)]
    dbg!(result);
    cb.map(|log| log(format!("[flash_only]result={result:?};")));
}
