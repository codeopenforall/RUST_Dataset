struct SafeData {
    ptr: Option<*mut i32>,
}
impl SafeData {
    fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        SafeData { ptr: Some(Box::into_raw(boxed)) }
    }
    fn get(&self) -> i32 {
        unsafe { *self.ptr.expect("Pointer should be valid") }
    }
}
impl Drop for SafeData {
    fn drop(&mut self) {
        if let Some(p) = self.ptr.take() {
            unsafe {
                let _ = Box::from_raw(p);
            }
        }
    }
}
fn run_app() -> i32 {
    let instance = SafeData::new(100);
    instance.get()
}
fn main() {
    let _ = run_app();
}
