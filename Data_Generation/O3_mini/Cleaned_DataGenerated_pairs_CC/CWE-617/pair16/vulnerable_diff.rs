use std::ptr;
        assert!(self.threshold < 100, "Threshold too high, possible DoS");
    }
}
static mut GLOBAL_SYSTEM: *mut System = ptr::null_mut();
fn load_configuration(value: usize) {
    unsafe {
        GLOBAL_SYSTEM = Box::into_raw(Box::new(System::new(value)));
    }
}
fn get_configuration() -> &'static System {
    unsafe {
        if GLOBAL_SYSTEM.is_null() {
            panic!("Configuration not loaded");
        }
        &*GLOBAL_SYSTEM
    load_configuration(value);
    let handle = thread::spawn(|| {
        unsafe {
            thread::sleep(Duration::from_millis(50));
            let config = get_configuration();
            config.process();
        }
    unsafe {
        let config = get_configuration();
        config.process();
