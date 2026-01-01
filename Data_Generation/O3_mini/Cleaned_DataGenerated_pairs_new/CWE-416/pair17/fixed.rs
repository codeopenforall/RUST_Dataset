struct Data {
    value: i32,
}
impl Data {
    fn new(val: i32) -> Self {
        Data { value: val }
    }
}
fn calculate() -> i32 {
    let boxed = Box::new(Data::new(42));
    let ptr = Box::into_raw(boxed);
    unsafe {
        let result = (*ptr).value;
        Box::from_raw(ptr);
        result
    }
}
fn main() {
    let result = calculate();
    println!("Result: {}", result);
}
