use std::ptr;
struct Resource {
    ptr: *mut i32,
}
impl Resource {
    unsafe fn obtain(&self) -> i32 {
        *self.ptr
    }
}
fn allocate() -> Resource {
    let boxed = Box::new(42);
    let raw = Box::into_raw(boxed);
    unsafe { Box::from_raw(raw) };
    Resource { ptr: raw }
}
fn execute() {
    let resource = allocate();
    unsafe {
        let val = resource.obtain();
        println!("Value: {}", val);
    }
}
fn main() {
    execute();
}
