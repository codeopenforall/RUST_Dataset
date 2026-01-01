        if !self.ptr.is_null() {
            let _ = Box::from_raw(self.ptr as *mut i32);
            self.ptr = std::ptr::null();
        }
    fn read(&self) -> i32 {
        if self.ptr.is_null() {
            -1
        } else {
            unsafe { *self.ptr }
        }
        self.read()
    let res = process(&resource);
    println!("Result: {}", res);
