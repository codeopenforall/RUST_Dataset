use std::ptr;
struct DataHolder {
    ptr: *mut i32,
}
impl DataHolder {
    fn new(value: i32) -> Self {
        let boxed = Box::new(value);
        Self { ptr: Box::into_raw(boxed) }
    }
    unsafe fn release(&mut self) {
        let _ = Box::from_raw(self.ptr);
    }
    unsafe fn read(&self) -> i32 {
        *self.ptr
    }
}
pub fn compute_value() -> i32 {
    let mut holder = DataHolder::new(123);
    unsafe {
        holder.release();
        let _dummy = Box::new(456);
        holder.read()
    }
}
fn main() {
    let res = compute_value();
    println!("Computed result: {}", res);
}
