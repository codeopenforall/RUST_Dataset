use std::sync::atomic::{AtomicBool, Ordering};
static FREED: AtomicBool = AtomicBool::new(false);
struct Resource {
    value: i32,
}
impl Drop for Resource {
    fn drop(&mut self) {
        if FREED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            panic!("double free detected");
        }
    }
}
fn unsafe_release() {
    let mem = Box::new(Resource { value: 42 });
    let raw_ptr = Box::into_raw(mem);
    unsafe {
        let _first_owner = Box::from_raw(raw_ptr);
        let _second_owner = Box::from_raw(raw_ptr); 
    }
}
fn run() {
    unsafe_release();
}
fn main() {
    run();
}
