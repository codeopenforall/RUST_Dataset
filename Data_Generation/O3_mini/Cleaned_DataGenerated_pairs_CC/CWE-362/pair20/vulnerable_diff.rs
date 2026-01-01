use std::cell::UnsafeCell;
    flag: UnsafeCell<bool>,
unsafe impl Sync for SharedState {}
        SharedState { flag: UnsafeCell::new(false) }
    unsafe fn check_then_set(&self) -> bool {
        if !*self.flag.get() {
            *self.flag.get() = true;
        handles.push(thread::spawn(move || unsafe { s.check_then_set() }));
