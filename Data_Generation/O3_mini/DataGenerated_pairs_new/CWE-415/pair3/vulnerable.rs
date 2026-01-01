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

/// Unsafely deallocates memory pointed by ptr. If already deallocated,
/// a double free is detected and a panic is triggered.
unsafe fn deallocate(ptr: *mut Data) {
    // simulate double free detection
    if FREED.swap(true, Ordering::SeqCst) {
        panic!("Double free detected!");
    }
    // manually free the resource by reconstructing the Box.
    Box::from_raw(ptr);
}

/// Allocates a new Data in the heap and returns a raw pointer
fn obtain_pointer() -> *mut Data {
    let data = Box::new(Data::new(42));
    Box::into_raw(data)
}

/// Executes a process that mistakenly deallocates the same pointer twice.
fn process() {
    let raw = obtain_pointer();
    unsafe {
        // First deallocation of the resource.
        deallocate(raw);
        // BUG: accidental second deallocation causing a double free.
        deallocate(raw);
    }
}

fn main() {
    process();
}