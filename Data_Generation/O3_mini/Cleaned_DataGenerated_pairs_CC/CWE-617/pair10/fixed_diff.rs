use std::sync::atomic::{AtomicI32, Ordering};
static SHARED: AtomicI32 = AtomicI32::new(0);
                loop {
                    let current = SHARED.load(Ordering::SeqCst);
                    if current + d >= THRESHOLD {
                        break;
                    }
                    if SHARED.compare_exchange(current, current + d, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                        break;
                    }
    let final_val = SHARED.load(Ordering::SeqCst);
    assert!(final_val < THRESHOLD, "Safe limit exceeded unexpectedly!");
    println!("Completed run with SHARED value: {}", final_val);
