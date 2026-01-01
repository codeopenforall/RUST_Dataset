struct Resource {
    data: i32,
}
static mut FREED: bool = false;
unsafe fn deallocate_once(ptr: *mut Resource) {
    if !FREED {
        FREED = true;
        let _ = Box::from_raw(ptr);
    }
}
fn execute() {
    let resource = Box::new(Resource { data: 42 });
    let raw_ptr = Box::into_raw(resource);
    unsafe {
        deallocate_once(raw_ptr);
    }
}
fn main() {
    execute();
}
