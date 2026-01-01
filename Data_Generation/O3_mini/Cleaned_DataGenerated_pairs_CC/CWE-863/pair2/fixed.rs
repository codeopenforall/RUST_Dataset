use std::thread;
pub struct Manager;
impl Manager {
    pub fn allocate_resources(role: &str, count: usize) -> Result<usize, String> {
        if role != "admin" {
            return Err("Unauthorized: only admin users can allocate resources.".to_string());
        }
        const MAX_RESOURCES: usize = 50;
        if count > MAX_RESOURCES {
            return Err(format!(
                "Requested count ({}) exceeds the maximum allowed resources ({}).",
                count, MAX_RESOURCES
            ));
        }
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
    match Manager::allocate_resources("user", 10) {
        Ok(num) => println!("Allocated {} resources successfully.", num),
        Err(err) => eprintln!("Error: {}", err),
    }
}
