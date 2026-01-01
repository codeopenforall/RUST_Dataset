use std::thread;
fn compute_value() -> i32 {
    let data = vec![1, 2, 3, 4, 5];                
    data.iter().sum()
}
struct Worker;
impl Worker {
    fn process(&self) -> i32 {
        compute_value()
    }
}
fn main() {
    let handler = Worker;
    let handle = thread::spawn(move || {
        handler.process()
    });
    let result = handle.join().unwrap();
    println!("Total: {}", result);
}
