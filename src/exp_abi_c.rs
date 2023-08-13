use ::std::ffi::{c_char, c_uint, CString};
use crate::{GitEdition, start_flash_by_ppid, start_flash_by_title, stop_flash_by_ppid, stop_flash_by_title};
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
