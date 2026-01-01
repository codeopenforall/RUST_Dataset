use std::ptr;
struct Owner {
    ptr: *mut Data,
}
impl Owner {
    fn new(val: u32) -> Self {
        let boxed = Box::new(Data { value: val });
        Owner { ptr: Box::into_raw(boxed) }
    unsafe fn release(&mut self) {
        if !self.ptr.is_null() {
            Box::from_raw(self.ptr);
            self.ptr = ptr::null_mut();
        }
    }
}
impl Drop for Owner {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                Box::from_raw(self.ptr);
                self.ptr = ptr::null_mut();
            }
        }
    }
    let mut owner = Owner::new(42);
        owner.release();
