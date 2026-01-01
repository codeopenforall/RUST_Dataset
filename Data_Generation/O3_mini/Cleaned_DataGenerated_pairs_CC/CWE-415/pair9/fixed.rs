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
    fn from_value(val: i32) -> Self {
        let boxed = Box::new(val);
        Item { data: Box::into_raw(boxed) }
    }
}
impl Clone for Item {
    fn clone(&self) -> Self {
        let val = self.value();
        Item::from_value(val)
    }
}
impl Drop for Item {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.data));
        }
    }
}
fn run_app() {
    let first = Item::new();
    let second = first.clone();
    println!("Values: {} and {}", first.value(), second.value());
}
fn main() {
    run_app();
}
