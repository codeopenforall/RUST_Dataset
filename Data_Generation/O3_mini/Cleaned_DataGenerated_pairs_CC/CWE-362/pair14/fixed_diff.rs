use std::sync::atomic::{AtomicU32, Ordering};
    count: AtomicU32,
        Shared { count: AtomicU32::new(0) }
        if self.count.compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            thread::sleep(Duration::from_millis(50));
        self.count.load(Ordering::SeqCst)
