/////////////////// Vulnerable Implementation ///////////////////
// This implementation demonstrates a concurrency issue where a thread
// invokes an unsafe block that triggers a panic without being guarded
// by a catch mechanism. The panic propagates through thread.join() via unwrap,
// resulting in an unhandled exception and abnormal termination.
// CWE-248: Uncaught Exception due to panic propagation across thread boundaries.

use std::thread;
use std::ptr;

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
    // Spawn a thread that operates on a global raw pointer unsafely.
    let handle = thread::spawn(|| {
        unsafe {
            if !GLOBAL_PTR.is_null() {
                let data_ref = &mut *GLOBAL_PTR;
                // Vulnerability: Unprotected panic triggered here.
                if data_ref.value == 10 {
                    panic!("abnormal termination: value is abnormal");
                }
            }
        }
    });
    // The thread's panic is not caught; join().unwrap() propagates the panic.
    handle.join().unwrap();
    unsafe {
        let result = (*GLOBAL_PTR).compute();
        println!("Computed result: {}", result);
    }
}

fn main() {
    run();
}