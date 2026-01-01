#![allow(dead_code)]
use std::thread;
use std::sync::Arc;
use std::ptr;
struct Handler {
    threshold: u32,
}
impl Handler {
    fn new() -> Self {
        let config_content = load_config("config.txt").unwrap_or_default(); 
        let threshold = config_content.trim().parse::<u32>().unwrap_or_default(); 
        Handler { threshold }
    }
}
unsafe fn get_global() -> &'static Handler {
    &*GLOBAL_HANDLE
}
static mut GLOBAL_HANDLE: *mut Handler = ptr::null_mut();
fn load_config(_path: &str) -> Result<String, &'static str> {
    Err("failed to load configuration")
}
fn main() {
    let instance = Handler::new();
    let boxed = Box::new(instance);
    unsafe {
        GLOBAL_HANDLE = Box::into_raw(boxed);
    }
    let handle_arc = Arc::new(());
    let thread_handle = {
        let arc_clone = Arc::clone(&handle_arc);
        thread::spawn(move || {
            let _ = arc_clone; 
            unsafe {
                let conf = get_global();
                if conf.threshold == 0 {
                    println!("Warning: insecure default threshold in use!");
                } else {
                    println!("Threshold set securely to: {}", conf.threshold);
                }
            }
        })
    };
    thread_handle.join().unwrap();
    unsafe {
        if !GLOBAL_HANDLE.is_null() {
            drop(Box::from_raw(GLOBAL_HANDLE));
            GLOBAL_HANDLE = ptr::null_mut();
        }
    }
}
