struct Data {
    ptr: *mut i32,
}
impl Data {
    fn new(val: i32) -> Self {
        let b = Box::new(val);
        Self { ptr: Box::into_raw(b) }
    }
}
impl Clone for Data {
    fn clone(&self) -> Self {
        unsafe {
            let value = *self.ptr;
            Data::new(value)
        }
    }
}
impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.ptr);
        }
    }
}
fn execute() {
    let original = Data::new(42);
    let duplicate = original.clone();
    let sum = unsafe { *original.ptr + *duplicate.ptr };
    println!("Sum: {}", sum);
}
fn main() {
    execute();
}
