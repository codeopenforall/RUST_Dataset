use std::{sync::atomic::{AtomicBool, AtomicU32, Ordering}, sync::Arc, thread, time::Duration};
    flag: AtomicBool,
    init_count: AtomicU32,
            flag: AtomicBool::new(false),
            init_count: AtomicU32::new(0),
        if !self.flag.load(Ordering::Acquire) {
            thread::sleep(Duration::from_millis(10));
            if self.flag.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                self.init_count.fetch_add(1, Ordering::AcqRel);
        self.init_count.load(Ordering::Relaxed)
