use std::thread;
struct Data {
    value: u32,
}
fn compute() -> u32 {
    let resource = Box::new(Data { value: 100 });
    let extracted = resource.value;
    drop(resource);
    let handle = thread::spawn(|| {
        let _ = Box::new(Data { value: 200 });
    });
    handle.join().unwrap();
    extracted
}
fn main() {
    let result = compute();
    println!("{}", result);
}
