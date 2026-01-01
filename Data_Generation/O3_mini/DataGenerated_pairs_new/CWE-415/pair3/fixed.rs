use std::sync::atomic::{AtomicBool, Ordering};
use std::ptr;
use std::mem;
use std::boxed::Box;

static FREED: AtomicBool = AtomicBool::new(false);

pub struct Data {
    pub value: i32,
}

impl Data {
    pub fn new(val: i32) -> Self {
        Data { value: val }
    }
}

/// Safely deallocates memory pointed by ptr. If already deallocated, a panic is triggered,
/// but correct usage guarantees a single deallocation.
unsafe fn deallocate(ptr: *mut Data) {
    if FREED.swap(true, Ordering::SeqCst) {
        panic!("Double free detected!");
    }
    Box::from_raw(ptr);
}

/// Allocates a new Data in the heap and returns a raw pointer.
fn obtain_pointer() -> *mut Data {
    let data = Box::new(Data::new(42));
    Box::into_raw(data)
}

/// Executes a process that correctly deallocates the resource only once.
fn process() {
    let raw = obtain_pointer();
    unsafe {
        // Correct usage: deallocate only once.
        deallocate(raw);
    }
}

fn main() {
    process();
}