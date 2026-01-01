////////////////////////////////////////////////////////////
// This corrected version introduces a strict upper limit to
// the number of tasks that may be spawned. Before launching
// the tasks, execute_request validates the requested count
// against a predetermined safe limit. It also replaces the
// unsafe global counter with a thread-safe atomic counter.
////////////////////////////////////////////////////////////

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

const SAFE_LIMIT: usize = 500;
static TASK_COUNT: AtomicUsize = AtomicUsize::new(0);

struct ResourceService;

impl ResourceService {
    pub fn execute_request(&self, num: usize) -> Result<(), String> {
        if num > SAFE_LIMIT {
            return Err(format!(
                "Requested number {} exceeds safe limit {}",
                num, SAFE_LIMIT
            ));
        }
        let mut handles = Vec::new();
        for _ in 0..num {
            handles.push(thread::spawn(|| {
                TASK_COUNT.fetch_add(1, Ordering::SeqCst);
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
        Ok(())
    }
}

fn main() {
    let service = ResourceService;
    match service.execute_request(1000) {
        Ok(_) => println!(
            "Total tasks processed: {}",
            TASK_COUNT.load(Ordering::SeqCst)
        ),
        Err(e) => eprintln!("Error: {}", e),
    }
}