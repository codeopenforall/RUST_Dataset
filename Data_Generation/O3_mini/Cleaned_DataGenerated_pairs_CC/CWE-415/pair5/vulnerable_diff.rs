use std::sync::atomic::{AtomicBool, Ordering};
use std::panic;
static FREED: AtomicBool = AtomicBool::new(false);
        Data { ptr: self.ptr }
        if FREED.swap(true, Ordering::SeqCst) {
            panic!("double free detected");
        }
    let result = panic::catch_unwind(|| {
        execute();
    });
    if result.is_err() {
        eprintln!("Error encountered during execution.");
    }
