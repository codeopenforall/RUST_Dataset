use std::sync::atomic::{AtomicBool, Ordering};
static FREED: AtomicBool = AtomicBool::new(false);
struct Item {
    data: *mut i32,
}
impl Item {
    fn new() -> Self {
        let boxed = Box::new(100);
        Item { data: Box::into_raw(boxed) }
    }
    fn value(&self) -> i32 {
        unsafe { *self.data }
    }
}
impl Clone for Item {
    fn clone(&self) -> Self {
        Item { data: self.data }
    }
}
impl Drop for Item {
    fn drop(&mut self) {
        unsafe {
            if FREED.load(Ordering::SeqCst) {
                panic!("Double free detected");
            } else {
                FREED.store(true, Ordering::SeqCst);
                drop(Box::from_raw(self.data));
            }
        }
    }
}
fn run_app() {
    FREED.store(false, Ordering::SeqCst);
    let first = Item::new();
    let second = first.clone();
    println!("Values: {} and {}", first.value(), second.value());
}
fn main() {
    run_app();
}
