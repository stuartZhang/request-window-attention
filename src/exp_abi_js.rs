mod nbg;

use ::node_bindgen::{derive::node_bindgen, init::node_bindgen_init_once};
use ::std::{cell::RefCell, thread_local};
use crate::{GitEdition, start_flash_by_ppid, start_flash_by_title, stop_flash_by_ppid, stop_flash_by_title};
use nbg::JsGlobalCallbackFunction;

thread_local! {
    static LOGGER_CALLBACK: RefCell<Option<Box<dyn Fn(String)>>> = RefCell::new(None);
}
#[node_bindgen_init_once]
fn main() {
    println!("{}", GitEdition::default());
}
#[node_bindgen]
fn set_logger(cb: Option<JsGlobalCallbackFunction>){
    let cb = cb.map(|cb| {
        Box::new(move |text: String| {
            cb.call(vec![text]).unwrap();
        }) as Box<dyn Fn(String)>
    });
    LOGGER_CALLBACK.with(move |logger_callback| {
        *logger_callback.borrow_mut() = cb;
    });
}
#[node_bindgen]
fn unset_logger(){
    LOGGER_CALLBACK.with(|logger_callback| {
        logger_callback.borrow_mut().take();
    });
}
#[node_bindgen]
fn stop_flash_by_title_js(win_title: String) {
    LOGGER_CALLBACK.with(|logger_callback| {
        stop_flash_by_title(win_title.into(), logger_callback.borrow().as_ref())
    })
}
#[node_bindgen]
fn start_flash_by_title_js(win_title: String, count: u32, blink_rate: u32) {
    LOGGER_CALLBACK.with(|logger_callback| {
        start_flash_by_title(win_title.into(), count, blink_rate, logger_callback.borrow().as_ref())
    })
}
#[node_bindgen]
fn stop_flash_by_ppid_js(process_id: i32) {
    LOGGER_CALLBACK.with(|logger_callback| {
        stop_flash_by_ppid(process_id as u32, logger_callback.borrow().as_ref())
    })
}
#[node_bindgen]
fn start_flash_by_ppid_js(process_id: i32, count: u32, blink_rate: u32) {
    LOGGER_CALLBACK.with(|logger_callback| {
        start_flash_by_ppid(process_id as u32, count, blink_rate, logger_callback.borrow().as_ref())
    })
}
#[node_bindgen]
fn get_edition() -> GitEdition {
    GitEdition::default()
}
