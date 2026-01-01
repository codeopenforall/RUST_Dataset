//////////////////////////////////////////////////////////////////////////////////////////////////////
// This code simulates an arithmetic module that uses unsafe operations and concurrency.
// It does not check for a zero divisor before performing division.
// The unsafe block dereferences a raw pointer to obtain the divisor, leading to a potential
// division-by-zero panic when b is zero. A secondary public API (run_calc) is provided for
// test orchestration (it wraps the core computation in a catch_unwind), but the main routine
// uses the raw computation directly.
//////////////////////////////////////////////////////////////////////////////////////////////////////
use std::sync::mpsc;
use std::thread;
use std::panic;

struct Module;

trait Operation {
    // Unsafe operation that performs division without checking divisor validity.
    unsafe fn transform(&self, a: i32, b: i32) -> i32;
}

impl Operation for Module {
    unsafe fn transform(&self, a: i32, b: i32) -> i32 {
        // The vulnerability occurs here: raw pointer is used to obtain b without check.
        let ptr: *const i32 = &b;
        let factor = *ptr;
        // No check: division by zero will panic if factor == 0.
        a / factor
    }
}

// Core computation that uses the unsafe transform.
fn compute(a: i32, b: i32) -> i32 {
    let m = Module;
    unsafe { m.transform(a, b) }
}

// Public API for testing purposes.
// It wraps the unsafe computation with catch_unwind to allow testing frameworks to
// inspect the outcome instead of letting the panic escape.
pub fn run_calc(a: i32, b: i32) -> Result<i32, &'static str> {
    let res = panic::catch_unwind(|| compute(a, b));
    match res {
        Ok(val) => Ok(val),
        Err(_) => Err("Division by zero occurred"),
    }
}

fn main_thread() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        // This call triggers the unsafe division without validating b.
        // When b==0, the division panics.
        let r = compute(10, 0);
        tx.send(r).unwrap();
    });
    // The receive will likely not complete because the spawned thread panics.
    let _ = rx.recv().unwrap();
    handle.join().unwrap();
}

fn main() {
    main_thread();
}