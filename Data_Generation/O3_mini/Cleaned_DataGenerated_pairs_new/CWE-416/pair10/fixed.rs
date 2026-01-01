use std::thread;
use std::time::Duration;
#[derive(Debug)]
struct Record {
    value: i32,
}
impl Record {
    fn new(val: i32) -> Self {
        Record { value: val }
    }
}
fn compute() -> i32 {
    let boxed = Box::new(Record::new(42));
    let value = boxed.value;
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(10));
        let _temp = Box::new(Record::new(100));
    });
    handle.join().unwrap();
    value
}
fn main() {
    let result = compute();
    println!("Result: {}", result);
}
