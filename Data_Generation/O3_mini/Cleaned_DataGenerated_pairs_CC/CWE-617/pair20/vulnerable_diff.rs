use std::ptr;
    unsafe fn modify(&mut self, val: u32) {
        let ptr = self as *mut Data;
        (*ptr).threshold = val;
            unsafe {
                data_guard.modify(input);
            }
