use std::thread;
use std::time::Duration;
struct Data {
    count: usize,
}
static mut GLOBAL: *mut Data = std::ptr::null_mut();
fn init() {
    let data = Box::new(Data { count: 0 });
    unsafe {
        GLOBAL = Box::into_raw(data);
    }
}
pub fn run_process() -> usize {
    init();
    let mut handles = Vec::new();
    for _ in 0..10 {
        handles.push(thread::spawn(|| {
            unsafe {
                let data = GLOBAL.as_mut().expect("Not initialized");
                if data.count % 2 == 0 {
                    thread::sleep(Duration::from_millis(10));
                    data.count += 1;
                } else {
                    thread::sleep(Duration::from_millis(10));
                    data.count += 2;
                }
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    unsafe {
        let final_data = &*GLOBAL;
        final_data.count
    }
}
fn main() {
    let result = run_process();
    println!("Final count: {}", result);
}
