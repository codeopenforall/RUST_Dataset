use std::sync::{Arc, Barrier};
static mut GLOBAL_QUEUE: *mut Vec<i32> = std::ptr::null_mut();
    unsafe {
        if task % 2 == 0 {
            let p: *const i32 = &task;
            let v = *p;
            let mut temp = task;
            temp += v;
        }
fn expand_queue(value: i32) {
    unsafe {
        if !GLOBAL_QUEUE.is_null() {
            (*GLOBAL_QUEUE).push(value);
        }
    let mut local_queue = Vec::<i32>::new();
    unsafe {
        GLOBAL_QUEUE = &mut local_queue as *mut _;
    }
                expand_queue(val);
    unsafe { (*GLOBAL_QUEUE).len() }
