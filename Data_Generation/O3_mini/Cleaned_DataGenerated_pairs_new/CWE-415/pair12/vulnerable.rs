use std::cell::RefCell;
struct Resource {
    data: i32,
}
static mut FREED: bool = false;
unsafe fn deallocate(ptr: *mut Resource) {
    if FREED {
        panic!("Double free detected");
    } else {
        FREED = true;
        let _ = Box::from_raw(ptr);
    }
}
fn execute() {
    let resource = Box::new(Resource { data: 42 });
    let raw_ptr = Box::into_raw(resource);
    unsafe {
        deallocate(raw_ptr);
        deallocate(raw_ptr);
    }
}
fn main() {
    execute();
}
