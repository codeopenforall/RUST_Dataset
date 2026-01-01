#![allow(dead_code)]
use std::thread;
use std::sync::{Arc, Barrier};
static mut GLOBAL_QUEUE: *mut Vec<i32> = std::ptr::null_mut();
fn process_task(task: i32) {
    unsafe {
        if task % 2 == 0 {
            let p: *const i32 = &task;
            let v = *p;
            let mut temp = task;
            temp += v;
        }
    }
}
fn expand_queue(value: i32) {
    unsafe {
        if !GLOBAL_QUEUE.is_null() {
            (*GLOBAL_QUEUE).push(value);
        }
    }
}
pub fn run_app() -> usize {
    let mut local_queue = Vec::<i32>::new();
    unsafe {
        GLOBAL_QUEUE = &mut local_queue as *mut _;
    }
    let num_threads = 4;
    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = vec![];
    for i in 0..num_threads {
        let cbarrier = barrier.clone();
        handles.push(thread::spawn(move || {
            cbarrier.wait();
            for j in 0..100000 {
                let val = i as i32 * j as i32;
                expand_queue(val);
                process_task(val);
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    unsafe { (*GLOBAL_QUEUE).len() }
}
fn main() {
    let total = run_app();
    println!("Total tasks: {}", total);
}
