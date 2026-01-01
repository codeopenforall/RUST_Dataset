use std::ptr;
struct Data {
    value: i32,
}
impl Data {
    fn new(val: i32) -> Self {
        Data { value: val }
    }
}
fn calculate() -> i32 {
    unsafe {
        let ptr = Box::into_raw(Box::new(Data::new(42)));
        Box::from_raw(ptr);
        ptr::write(ptr, Data::new(999));
        (*ptr).value
    }
}
fn main() {
    let result = calculate();
    println!("Result: {}", result);
}
