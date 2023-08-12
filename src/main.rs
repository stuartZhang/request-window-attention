use ::std::{thread, time::Duration};
fn main() {
    request_window_attention::start_flash("有道云笔记".into(), 10, 500);
    thread::sleep(Duration::from_secs(2));
    request_window_attention::stop_flash("有道云笔记".into());
}