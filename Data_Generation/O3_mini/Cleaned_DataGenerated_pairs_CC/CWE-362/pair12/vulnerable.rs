use std::thread;
static mut GLOBAL: u32 = 0;
fn run() -> u32 {
    unsafe {
        GLOBAL = 0;
    }
    let mut handles = vec![];
    for _ in 0..10 {
        handles.push(thread::spawn(|| {
            for _ in 0..1000 {
                unsafe {
                    let temp = GLOBAL;          
                    let new_val = temp.wrapping_add(1); 
                    thread::yield_now();        
                    GLOBAL = new_val;           
                }
            }
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
    unsafe { GLOBAL }
}
fn main() {
    let result = run();
    println!("Final counter value: {}", result);
}
