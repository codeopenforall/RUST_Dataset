use std::sync::atomic::{AtomicBool, Ordering};
static FREED: AtomicBool = AtomicBool::new(false);
struct Resource {
    value: i32,
}
impl Drop for Resource {
    fn drop(&mut self) {
        if FREED.swap(true, Ordering::SeqCst) {
            panic!("Double free detected in drop!");
        }
    }
}
fn process() {
    let boxed = Box::new(Resource { value: 42 });
    let raw_ptr = Box::into_raw(boxed);
    unsafe {
        let recovered = Box::from_raw(raw_ptr);
        drop(recovered);
    }
}
fn main() {
    process();
    println!("Completed corrected main function");
}
