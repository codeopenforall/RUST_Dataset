//////////////////////////////////////////////////////////////
// The improved program adds resource limits and access control.
// Only users with the "admin" role are allowed to allocate resources,
// and even then the allocation count is capped to prevent abuse.
//////////////////////////////////////////////////////////////

use std::thread;

pub struct Manager;

impl Manager {
    // This function now enforces role-based access control and limits
    // the number of resources that can be allocated.
    pub fn allocate_resources(role: &str, count: usize) -> Result<usize, String> {
        // Enforce that only admins can allocate resources.
        if role != "admin" {
            return Err("Unauthorized: only admin users can allocate resources.".to_string());
        }
        // Cap the maximum allowed count to prevent resource exhaustion.
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
                    // Unsafe block is still used for low-level memory operations.
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
    // For demonstration, try to allocate resources with non-admin role.
    // In fixed version, this should result in an error.
    match Manager::allocate_resources("user", 10) {
        Ok(num) => println!("Allocated {} resources successfully.", num),
        Err(err) => eprintln!("Error: {}", err),
    }
}