///////////////////////////////////////////////
// CWE-670 Example: Corrected Control Flow 
//
// This corrected code retains the same unsafe and concurrent
// design to simulate a real-world scenario but fixes the branch
// logic in the task's run method. High-priority tasks now correctly
// multiply by 2 and low-priority tasks add 5 as intended.
///////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Level {
    High,
    Low,
}

trait Compute {
    fn run(&self) -> i32;
}

struct Task {
    num: i32,
    lev: Level,
}

impl Compute for Task {
    fn run(&self) -> i32 {
        // Corrected control-flow with proper match arms.
        unsafe {
            let ptr = &self.num as *const i32;
            let value = *ptr;
            match self.lev {
                Level::High => {
                    // Correct: high priority multiplies by 2.
                    return value.wrapping_mul(2);
                }
                Level::Low => {
                    // Correct: low priority adds 5.
                    return value + 5;
                }
            }
        }
    }
}

// Helper function that executes tasks concurrently.
pub fn execute_tasks() -> Vec<i32> {
    let tasks = Arc::new(Mutex::new(vec![
        Task { num: 20, lev: Level::High }, // Expected: 20*2 = 40.
        Task { num: 10, lev: Level::Low },  // Expected: 10+5 = 15.
    ]));

    let mut handles = vec![];

    // Spawn two threads processing the same tasks.
    for _ in 0..2 {
        let tasks_cloned = Arc::clone(&tasks);
        let handle = thread::spawn(move || {
            let mut results = Vec::new();
            let jobs = tasks_cloned.lock().unwrap();
            for job in jobs.iter() {
                results.push(job.run());
            }
            results
        });
        handles.push(handle);
    }

    let mut final_results = Vec::new();
    for handle in handles {
        let thread_results = handle.join().unwrap();
        final_results.extend(thread_results);
    }
    final_results
}

fn main() {
    let output = execute_tasks();
    println!("Output: {:?}", output);
}