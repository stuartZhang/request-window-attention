#![cfg_attr(debug_assertions, feature(trace_macros, log_syntax))]
mod git_edition;
#[cfg(any(feature = "nodejs", feature = "nw"))]
use ::node_bindgen::{derive::node_bindgen, init::node_bindgen_init_once};
use ::std::{ffi::{c_char, c_uint, CString, OsString}, iter, mem, os::windows::ffi::OsStrExt};
use ::winapi::{_core::ptr::null_mut, um::winuser::{FindWindowW, FlashWindowEx, FLASHWINFO, FLASHW_CAPTION, FLASHW_TIMER, FLASHW_TIMERNOFG, FLASHW_STOP, FLASHW_TRAY}};
pub use git_edition::GitEdition;
// Rust
pub fn stop_flash(win_title: OsString){
    flash(win_title, FLASHW_STOP, 0, 0)
}
pub fn start_flash(win_title: OsString, count: u32, blink_rate: u32){
    flash(win_title, FLASHW_CAPTION | FLASHW_TIMER | FLASHW_TIMERNOFG | FLASHW_TRAY, count, blink_rate)
}
fn flash(win_title: OsString, action: u32, count: u32, blink_rate: u32){
    let winname = win_title.encode_wide().chain(iter::once(0)).collect::<Vec<u16>>();
    let hwnd = unsafe { FindWindowW(null_mut(), winname.as_ptr()) };
    if hwnd.is_null() {
        return;
    }
    #[cfg(debug_assertions)]
    dbg!(hwnd);
    let mut flash_info = FLASHWINFO {
        cbSize: mem::size_of::<FLASHWINFO>() as u32,
        hwnd,
        dwFlags: action,
        uCount: count,
        dwTimeout: blink_rate
    };
    let _result = unsafe { FlashWindowEx(&mut flash_info) };
    #[cfg(debug_assertions)]
    dbg!(_result);
}
// C
/// 结束闪烁，但窗口任务栏还会继续高亮，直到窗体获得用户操作的焦点
/// @param winTitle 被闪烁窗体“标题名”
#[export_name = "stopFlashC"]
pub extern fn stop_flash_c(win_title: *const c_char) {
    let win_title = unsafe { CString::from_raw(win_title as *mut i8) }.into_string().unwrap();
    stop_flash(win_title.into())
}
/// /// 开始闪烁。
/// （1）在 stopFlashJs() 接口被调用后，闪烁会停止但高亮会继续。
/// （2）在窗体获得了焦点之后，闪烁与高亮才都会结束。
/// @param winTitle 被闪烁窗体“标题名”
/// @param blinkCount  闪烁次数。超过闪烁次数之后，任务栏会一直保持高亮状态。
/// @param blinkRate   相邻闪烁的间隔时间（单位：毫秒）
#[export_name = "startFlashC"]
pub extern fn start_flash_c(win_title: *const c_char, count: c_uint, blink_rate: c_uint) {
    let win_title = unsafe { CString::from_raw(win_title as *mut i8) }.into_string().unwrap();
    start_flash(win_title.into(), count, blink_rate)
}
/// 模块版本信息
#[export_name = "getEditionC"]
pub extern fn get_edition_c() -> *mut GitEdition {
    Box::into_raw(Box::new(GitEdition::default()))
}
// nodejs
#[cfg_attr(any(feature = "nodejs", feature = "nw"), node_bindgen_init_once)]
#[cfg(any(feature = "nodejs", feature = "nw"))]
fn main() {
    println!("{}", GitEdition::default());
}
#[cfg_attr(any(feature = "nodejs", feature = "nw"), node_bindgen)]
#[cfg(any(feature = "nodejs", feature = "nw"))]
fn stop_flash_js(win_title: String) {
    stop_flash(win_title.into())
}
#[cfg_attr(any(feature = "nodejs", feature = "nw"), node_bindgen)]
#[cfg(any(feature = "nodejs", feature = "nw"))]
fn start_flash_js(win_title: String, count: u32, blink_rate: u32) {
    start_flash(win_title.into(), count, blink_rate)
}
#[cfg_attr(any(feature = "nodejs", feature = "nw"), node_bindgen(name = "getEdition"))]
#[cfg(any(feature = "nodejs", feature = "nw"))]
fn get_edition() -> GitEdition {
    GitEdition::default()
}