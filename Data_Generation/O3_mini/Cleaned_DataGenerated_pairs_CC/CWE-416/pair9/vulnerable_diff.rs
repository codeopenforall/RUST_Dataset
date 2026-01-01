    ptr: *mut u32,
        let b = Box::new(val);
            ptr: Box::into_raw(b),
    pub unsafe fn compute(&self) -> u32 {
        *self.ptr
    }
}
impl Drop for Processor {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.ptr);
        }
    let result: u32;
    let local_ptr: *mut u32;
    {
        let job = Processor::new(42);
        local_ptr = job.ptr;
    } 
    unsafe {
        result = *local_ptr;
    }
    result
