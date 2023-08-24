#![cfg_attr(debug_assertions, feature(trace_macros, log_syntax))]
mod git_edition;
mod enum_windows;
#[cfg(any(feature = "nodejs", feature = "nw"))]
use ::node_bindgen::{core::val::JsCallbackFunction, derive::node_bindgen, init::node_bindgen_init_once};
use ::std::{ffi::{c_char, c_uint, CString, OsString}, iter, mem, os::windows::ffi::OsStrExt};
use ::winapi::{_core::ptr::null_mut, um::winuser::{FindWindowW, FlashWindowEx, FLASHWINFO, FLASHW_CAPTION, FLASHW_TIMER, FLASHW_TIMERNOFG, FLASHW_STOP, FLASHW_TRAY}, shared::windef::HWND};
pub use git_edition::GitEdition;
// ----------------------------------------------------------------------
// Rust
// ----------------------------------------------------------------------
pub fn stop_flash_by_title(win_title: OsString, cb: Option<Box<dyn Fn(String)>>) {
    flash_by_title(win_title, FLASHW_STOP, 0, 0, cb)
}
pub fn start_flash_by_title(win_title: OsString, count: u32, blink_rate: u32, cb: Option<Box<dyn Fn(String)>>) {
    flash_by_title(win_title, FLASHW_CAPTION | FLASHW_TIMER | FLASHW_TIMERNOFG | FLASHW_TRAY, count, blink_rate, cb)
}
fn flash_by_title(win_title: OsString, action: u32, count: u32, blink_rate: u32, cb: Option<Box<dyn Fn(String)>>){
    let winname = win_title.encode_wide().chain(iter::once(0)).collect::<Vec<u16>>();
    let hwnd = unsafe { FindWindowW(null_mut(), winname.as_ptr()) };
    if hwnd.is_null() {
        return;
    }
    flash_only(hwnd, action, count, blink_rate, cb.as_ref());
}
//
pub fn stop_flash_by_ppid(process_id: u32, cb: Option<Box<dyn Fn(String)>>){
    flash_by_ppid(process_id, FLASHW_STOP, 0, 0, cb)
}
pub fn start_flash_by_ppid(process_id: u32, count: u32, blink_rate: u32, cb: Option<Box<dyn Fn(String)>>) {
    flash_by_ppid(process_id, FLASHW_CAPTION | FLASHW_TIMER | FLASHW_TIMERNOFG | FLASHW_TRAY, count, blink_rate, cb)
}
fn flash_by_ppid(process_id: u32, action: u32, count: u32, blink_rate: u32, cb: Option<Box<dyn Fn(String)>>) {
    if let Some(hwnd) = enum_windows::enumerate_windows(process_id, cb.as_ref()) {
        flash_only(hwnd, action, count, blink_rate, cb.as_ref());
    }
}
//
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
// ----------------------------------------------------------------------
// C
// ----------------------------------------------------------------------
/// 结束闪烁，但窗口任务栏还会继续高亮，直到窗体获得用户操作的焦点
/// @param winTitle 被闪烁窗体“标题名”
#[export_name = "stopFlashByTitleC"]
pub extern fn stop_flash_by_title_c(win_title: *const c_char) {
    let win_title = unsafe { CString::from_raw(win_title as *mut i8) }.into_string().unwrap();
    stop_flash_by_title(win_title.into(), None)
}
/// /// 开始闪烁。
/// （1）在 stopFlashJs() 接口被调用后，闪烁会停止但高亮会继续。
/// （2）在窗体获得了焦点之后，闪烁与高亮才都会结束。
/// @param winTitle 被闪烁窗体“标题名”
/// @param blinkCount  闪烁次数。超过闪烁次数之后，任务栏会一直保持高亮状态。
/// @param blinkRate   相邻闪烁的间隔时间（单位：毫秒）
#[export_name = "startFlashByTitleC"]
pub extern fn start_flash_by_title_c(win_title: *const c_char, count: c_uint, blink_rate: c_uint) {
    let win_title = unsafe { CString::from_raw(win_title as *mut i8) }.into_string().unwrap();
    start_flash_by_title(win_title.into(), count, blink_rate, None)
}
/// 结束闪烁，但窗口任务栏还会继续高亮，直到窗体获得用户操作的焦点
/// @param process_id 被闪烁窗体的进程ID或nwjs的进程PID
#[export_name = "stopFlashByPpidC"]
pub extern fn stop_flash_by_ppid_c(process_id: c_uint) {
    stop_flash_by_ppid(process_id, None)
}
/// /// 开始闪烁。
/// （1）在 stopFlashJs() 接口被调用后，闪烁会停止但高亮会继续。
/// （2）在窗体获得了焦点之后，闪烁与高亮才都会结束。
/// @param process_id  被闪烁窗体的进程ID或nwjs的进程PID
/// @param blinkCount  闪烁次数。超过闪烁次数之后，任务栏会一直保持高亮状态。
/// @param blinkRate   相邻闪烁的间隔时间（单位：毫秒）
#[export_name = "startFlashByPpidC"]
pub extern fn start_flash_by_ppid_c(process_id: c_uint, count: c_uint, blink_rate: c_uint) {
    start_flash_by_ppid(process_id, count, blink_rate, None)
}
/// 模块版本信息
#[export_name = "getEditionC"]
pub extern fn get_edition_c() -> *mut GitEdition {
    Box::into_raw(Box::new(GitEdition::default()))
}
// ----------------------------------------------------------------------
// nodejs
// ----------------------------------------------------------------------
#[cfg_attr(any(feature = "nodejs", feature = "nw"), node_bindgen_init_once)]
#[cfg(any(feature = "nodejs", feature = "nw"))]
fn main() {
    println!("{}", GitEdition::default());
}
#[cfg_attr(any(feature = "nodejs", feature = "nw"), node_bindgen)]
#[cfg(any(feature = "nodejs", feature = "nw"))]
fn stop_flash_by_title_js(win_title: String, cb: Option<JsCallbackFunction>) {
    stop_flash_by_title(win_title.into(), build_callback(cb))
}
#[cfg_attr(any(feature = "nodejs", feature = "nw"), node_bindgen)]
#[cfg(any(feature = "nodejs", feature = "nw"))]
fn start_flash_by_title_js(win_title: String, count: u32, blink_rate: u32, cb: Option<JsCallbackFunction>) {
    start_flash_by_title(win_title.into(), count, blink_rate, build_callback(cb))
}
#[cfg_attr(any(feature = "nodejs", feature = "nw"), node_bindgen)]
#[cfg(any(feature = "nodejs", feature = "nw"))]
fn stop_flash_by_ppid_js(process_id: i32, cb: Option<JsCallbackFunction>) {
    stop_flash_by_ppid(process_id as u32, build_callback(cb))
}
#[cfg_attr(any(feature = "nodejs", feature = "nw"), node_bindgen)]
#[cfg(any(feature = "nodejs", feature = "nw"))]
fn start_flash_by_ppid_js(process_id: i32, count: u32, blink_rate: u32, cb: Option<JsCallbackFunction>) {
    start_flash_by_ppid(process_id as u32, count, blink_rate, build_callback(cb))
}
#[cfg_attr(any(feature = "nodejs", feature = "nw"), node_bindgen)]
#[cfg(any(feature = "nodejs", feature = "nw"))]
fn get_edition() -> GitEdition {
    GitEdition::default()
}
#[cfg(any(feature = "nodejs", feature = "nw"))]
fn build_callback(cb: Option<JsCallbackFunction>) -> Option<Box<dyn Fn(String)>> {
    cb.map(|cb| {
        Box::new(move |text: String| {
            cb.call(vec![text]).unwrap();
        }) as Box<dyn Fn(String)>
    })
}