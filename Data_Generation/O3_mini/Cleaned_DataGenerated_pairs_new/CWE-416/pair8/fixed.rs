struct DataHolder {
    ptr: Option<*mut i32>, 
}
impl DataHolder {
    fn new(value: i32) -> Self {
        let boxed = Box::new(value);
        Self { ptr: Some(Box::into_raw(boxed)) }
    }
    unsafe fn release(&mut self) -> i32 {
        if let Some(raw) = self.ptr.take() {
            let value = *raw;
            let _ = Box::from_raw(raw);
            value
        } else {
            panic!("Memory already released");
        }
    }
}
pub fn compute_value() -> i32 {
    let mut holder = DataHolder::new(123);
    unsafe {
        holder.release()
    }
}
fn main() {
    let res = compute_value();
    println!("Computed result: {}", res);
}
