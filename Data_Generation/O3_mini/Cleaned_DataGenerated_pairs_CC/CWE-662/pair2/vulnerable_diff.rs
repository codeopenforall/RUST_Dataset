    fn increment(&self) {
        unsafe {
            let ptr = self as *const Data as *mut Data;
            (*ptr).count = (*ptr).count.wrapping_add(1);
        }
        let guard = self.data.read().unwrap();
        drop(guard);
    let ctrl = Controller::new();
    let shared_ctrl = Arc::new(ctrl);
