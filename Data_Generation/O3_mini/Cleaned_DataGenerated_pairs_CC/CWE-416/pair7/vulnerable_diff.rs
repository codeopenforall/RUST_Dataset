    ptr: *mut i32,
        let b = Box::new(val);
        let ptr = Box::into_raw(b);
        Resource { ptr }
    unsafe fn dispose(&mut self) {
        let _ = Box::from_raw(self.ptr);
    }
    unsafe fn read(&self) -> i32 {
        *self.ptr
    let mut res = Resource::new(123);
    unsafe {
        res.dispose();                   
        let _dummy = Box::new(999);
        res.read()                       
    }
