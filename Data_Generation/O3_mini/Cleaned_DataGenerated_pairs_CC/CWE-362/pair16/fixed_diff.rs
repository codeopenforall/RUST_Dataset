use std::sync::atomic::{AtomicBool, Ordering};
    flag: AtomicBool,
        Self { flag: AtomicBool::new(true) }
        if self.flag.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            thread::sleep(Duration::from_micros(10));
            return true;
