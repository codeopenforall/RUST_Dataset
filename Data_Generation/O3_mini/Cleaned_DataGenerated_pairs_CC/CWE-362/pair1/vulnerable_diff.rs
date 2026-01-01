use std::ptr;
    counter: *mut usize,
        let boxed = Box::new(initial);
        Data { counter: Box::into_raw(boxed) }
    }
    fn increment(&self) {
        unsafe {
            let val = ptr::read_volatile(self.counter);
            thread::yield_now();
            let new_val = val + 1;
            ptr::write_volatile(self.counter, new_val);
    fn get(&self) -> usize {
        unsafe {
            ptr::read_volatile(self.counter)
        }
}
impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.counter);
        }
        let ptr_data = &data as *const Data;
                unsafe {
                    (*ptr_data).increment();
                }
