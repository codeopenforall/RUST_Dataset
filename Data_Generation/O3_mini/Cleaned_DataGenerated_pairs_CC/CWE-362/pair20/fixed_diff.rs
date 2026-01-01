use std::sync::atomic::{AtomicBool, Ordering};
    flag: AtomicBool,
        SharedState { flag: AtomicBool::new(false) }
    fn check_then_set(&self) -> bool {
        if self.flag.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
        handles.push(thread::spawn(move || s.check_then_set()));
