use ::std::{thread, time::Duration};
fn main() {
    request_window_attention::start_flash_by_title("有道云笔记".into(), 10, 500, None);
    thread::sleep(Duration::from_secs(2));
    request_window_attention::stop_flash_by_title("有道云笔记".into(), None);
    //
    request_window_attention::start_flash_by_ppid(3300, 10, 500, None);
    thread::sleep(Duration::from_secs(2));
    request_window_attention::stop_flash_by_ppid(3300, None);
}