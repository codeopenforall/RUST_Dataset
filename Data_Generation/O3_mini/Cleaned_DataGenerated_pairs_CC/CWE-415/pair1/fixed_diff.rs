struct SafeData {
    ptr: Option<*mut i32>,
impl SafeData {
        SafeData { ptr: Some(Box::into_raw(boxed)) }
    }
    fn get(&self) -> i32 {
        unsafe { *self.ptr.expect("Pointer should be valid") }
impl Drop for SafeData {
        if let Some(p) = self.ptr.take() {
            unsafe {
                let _ = Box::from_raw(p);
    let instance = SafeData::new(100);
    instance.get()
