use std::sync::atomic::{AtomicUsize, Ordering};
    unsafe fn release(&self);
    pub unsafe fn process(&self) {
static DEALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);
    unsafe fn release(&self) {
        let count = DEALLOC_COUNT.fetch_add(1, Ordering::SeqCst);
        if count >= 1 {
            panic!("Double free detected during explicit release (unsafe manual deallocation)!");
            let count = DEALLOC_COUNT.fetch_add(1, Ordering::SeqCst);
            if count >= 1 {
                panic!("Double free detected in destructor (Drop)!");
            let _ = Box::from_raw(self.ptr);
        let resource = Manager::initialize(100);
