use std::boxed::Box;
struct Resource {
    value: i32,
}
impl Resource {
    fn new(v: i32) -> Self {
        Resource { value: v }
    }
}
fn process() -> i32 {
    unsafe {
        let resource = Box::new(Resource::new(42));
        let raw_ptr: *mut Resource = Box::into_raw(resource);
        let result = (*raw_ptr).value;
        let _ = Box::from_raw(raw_ptr);
        result
    }
}
fn main() {
    let result = process();
    println!("Result: {}", result);
}
