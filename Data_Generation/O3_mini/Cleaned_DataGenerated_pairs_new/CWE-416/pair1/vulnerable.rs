use std::thread;
struct Data {
    value: u32,
}
fn compute() -> u32 {
    let resource = Box::new(Data { value: 100 });
    let ptr = Box::into_raw(resource);
    unsafe {
        let _ = Box::from_raw(ptr);
    }
    let handle = thread::spawn(|| {
        let _ = Box::new(Data { value: 200 });
    });
    handle.join().unwrap();
    unsafe { (*ptr).value }
}
fn main() {
    let result = compute();
    println!("{}", result);
}
