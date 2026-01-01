use std::sync::atomic::{AtomicBool, Ordering};
    data: *mut i32,
    flag: *mut AtomicBool,
        let data = Box::into_raw(Box::new(val));
        let flag = Box::into_raw(Box::new(AtomicBool::new(false)));
        Resource { data, flag }
        Resource {
            data: self.data,
            flag: self.flag,
        }
    }
}
impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            if (*self.flag).compare_and_swap(false, true, Ordering::SeqCst) == false {
                let _ = Box::from_raw(self.data);
                let _ = Box::from_raw(self.flag);
            } else {
                panic!("Double free detected");
            }
        }
