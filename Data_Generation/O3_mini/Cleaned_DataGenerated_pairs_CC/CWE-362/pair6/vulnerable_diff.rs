use std::cell::UnsafeCell;
    value: UnsafeCell<i32>,
unsafe impl Sync for Counter {}
        Counter { value: UnsafeCell::new(0) }
        unsafe { *self.value.get() }
        unsafe {
            let cur = *self.value.get();
            *self.value.get() = cur + delta;
        }
    let counter = Counter::new();
        let c = &counter;
