//////////////////// Fixed Implementation //////////////////////////
// In this revised version, the thread’s execution is wrapped with a catch_unwind,
// ensuring that any panic is caught and handled gracefully. Upon catching a panic,
// the code recovers by adjusting the state to avoid re-triggering the error, thereby
// preventing abnormal termination. The fix ensures that panics do not propagate unchecked.

use std::thread;
use std::ptr;
use std::panic;

#[derive(Debug)]
struct Data {
    value: u32,
}

trait Compute {
    fn compute(&self) -> u32;
}

impl Compute for Data {
    fn compute(&self) -> u32 {
        self.value * 2
    }
}

static mut GLOBAL_PTR: *mut Data = ptr::null_mut();

fn prepare() {
    let data = Box::new(Data { value: 10 });
    unsafe {
        GLOBAL_PTR = Box::into_raw(data);
    }
}

pub fn run() {
    prepare();
    // Spawn a thread and use catch_unwind to handle any potential panic.
    let handle = thread::spawn(|| {
        let result = panic::catch_unwind(|| {
            unsafe {
                if !GLOBAL_PTR.is_null() {
                    let data_ref = &mut *GLOBAL_PTR;
                    if data_ref.value == 10 {
                        panic!("abnormal termination: value is abnormal");
                    }
                }
            }
        });
        // Handle the panic by recovering and patching the state.
        if let Err(e) = result {
            println!("Caught panic in thread: {:?}", e);
            unsafe {
                if !GLOBAL_PTR.is_null() {
                    // Update the state so that future operations succeed.
                    (*GLOBAL_PTR).value = 20;
                }
            }
        }
    });
    // The join now does not propagate a panic.
    handle.join().unwrap();
    unsafe {
        let result = (*GLOBAL_PTR).compute();
        println!("Computed result: {}", result);
    }
}

fn main() {
    run();
}