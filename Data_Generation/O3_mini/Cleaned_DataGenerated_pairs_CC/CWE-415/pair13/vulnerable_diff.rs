use std::sync::atomic::{AtomicBool, Ordering};
static TRIGGERED: AtomicBool = AtomicBool::new(false);
        if TRIGGERED.swap(true, Ordering::SeqCst) {
            panic!("double free detected");
        }
        let box_b = Box::from_raw(raw_ptr);
        let sum = box_a.value + box_b.value;
        sum
