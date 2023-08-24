use ::winapi::{shared::{minwindef::{BOOL, FALSE, LPARAM, TRUE}, windef::HWND}, um::winuser::{EnumWindows, EnumChildWindows, GetWindowThreadProcessId}};
use ::std::{ffi::c_void, mem};

pub fn enumerate_windows(process_id: u32, log: Option<&Box<dyn Fn(String)>>) -> Option<HWND> {
    let mut result: Option<HWND> = None;
    let mut callback = |hwnd1: HWND| -> bool {
        let mut pid: u32 = 0;
        unsafe { GetWindowThreadProcessId(hwnd1, &mut pid) };
        #[cfg(debug_assertions)]
        dbg!("enumerate_windows", hwnd1, pid);
        log.map(|log| log(format!("[enumerate_windows] hwnd={hwnd1:?}; pid={pid}; process_id={process_id}")));
        if process_id == pid {
            result.replace(hwnd1);
            false
        } else if let Some(hwnd2) = enumerate_child_windows(hwnd1, process_id, log) {
            result.replace(hwnd2);
            false
        } else {
            true
        }
    };
    let mut trait_obj: &mut dyn FnMut(HWND) -> bool = &mut callback;
    let closure_pointer_pointer: *mut c_void = unsafe { mem::transmute(&mut trait_obj) };
    let lparam = closure_pointer_pointer as LPARAM;
    unsafe { EnumWindows(Some(enumerate_callback), lparam) };
    result
}
pub fn enumerate_child_windows(hwnd1: HWND, process_id: u32, log: Option<&Box<dyn Fn(String)>>) -> Option<HWND> {
    let mut result: Option<HWND> = None;
    let mut callback = |hwnd2: HWND| -> bool {
        let mut pid: u32 = 0;
        unsafe { GetWindowThreadProcessId(hwnd2, &mut pid) };
        #[cfg(debug_assertions)]
        dbg!("enumerate_child_windows", hwnd2, pid);
        log.map(|log| log(format!("[enumerate_child_windows] hwnd={hwnd2:?}; pid={pid}; process_id={process_id}")));
        if process_id == pid {
            result.replace(hwnd2);
            false
        } else {
            true
        }
    };
    let mut trait_obj: &mut dyn FnMut(HWND) -> bool = &mut callback;
    let closure_pointer_pointer: *mut c_void = unsafe { mem::transmute(&mut trait_obj) };
    let lparam = closure_pointer_pointer as LPARAM;
    unsafe { EnumChildWindows(hwnd1, Some(enumerate_callback), lparam) };
    result
}
unsafe extern "system" fn enumerate_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let closure: &mut &mut dyn FnMut(HWND) -> bool = mem::transmute(lparam as *mut c_void);
    if closure(hwnd) { TRUE } else { FALSE }
}
