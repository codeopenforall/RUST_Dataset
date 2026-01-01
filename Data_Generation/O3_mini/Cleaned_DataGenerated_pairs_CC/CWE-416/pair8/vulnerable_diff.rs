use std::ptr;
    ptr: *mut i32,
        Self { ptr: Box::into_raw(boxed) }
    unsafe fn release(&mut self) {
        let _ = Box::from_raw(self.ptr);
    }
    unsafe fn read(&self) -> i32 {
        *self.ptr
        holder.release();
        let _dummy = Box::new(456);
        holder.read()
