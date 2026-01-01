use std::alloc::{alloc, Layout};
use std::thread;
struct Data {
    value: i32,
}
fn compute() -> i32 {
    let b = Box::new(Data { value: 42 });
    let ptr = Box::into_raw(b);
    unsafe {
        drop(Box::from_raw(ptr));
    }
    let handle = thread::spawn(|| {
        let layout = Layout::new::<Data>();
        unsafe {
            let mem = alloc(layout) as *mut Data;
            *mem = Data { value: 99 };
        }
    });
    handle.join().unwrap();
    let result = unsafe { (*ptr).value };
    result
}
fn main() {
    let res = compute();
    println!("Result: {}", res);
}
