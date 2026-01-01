////////////////////////////////////////////////////////////
// This is a resource-service example that improperly allocates
// tasks without checking for a safe limit. In this version,
// the function execute_request spawns the requested number
// of threads and uses an unsafe global counter without any
// throttling. An attacker can trigger resource exhaustion by
// requesting an excessive number of tasks.
////////////////////////////////////////////////////////////

use std::thread;

static mut TASK_COUNT: usize = 0;

struct ResourceService;

impl ResourceService {
    pub fn execute_request(&self, num: usize) -> Result<(), String> {
        // No limit is enforced on the number of tasks spawned.
        let mut handles = Vec::new();
        for _ in 0..num {
            handles.push(thread::spawn(|| {
                // UNSAFE: Directly modifying a global variable without synchronization.
                unsafe {
                    TASK_COUNT += 1;
                }
            }));
        }
        for handle in handles {
            // Ignoring potential thread join errors for simplicity.
            let _ = handle.join();
        }
        Ok(())
    }
}

fn main() {
    let service = ResourceService;
    // Attempt to process a high number of tasks.
    if let Err(e) = service.execute_request(1000) {
        eprintln!("Error: {}", e);
    } else {
        unsafe {
            println!("Total tasks spawned: {}", TASK_COUNT);
        }
    }
}