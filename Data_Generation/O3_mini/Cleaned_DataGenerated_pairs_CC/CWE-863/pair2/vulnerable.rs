use std::thread;
pub struct Manager;
impl Manager {
    pub fn allocate_resources(role: &str, count: usize) -> Result<usize, String> {
        let mut handles = Vec::with_capacity(count);
        for _ in 0..count {
            let handle = thread::spawn(|| {
                unsafe {
                    let mut val: i32 = 0;
                    let ptr: *mut i32 = &mut val as *mut i32;
                    *ptr = 42;
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            let _ = handle.join();
        }
        Ok(count)
    }
}
fn main() {
    match Manager::allocate_resources("user", 1000) {
        Ok(num) => println!("Allocated {} resources successfully.", num),
        Err(err) => eprintln!("Error: {}", err),
    }
}
