///////////////////////////////////////////////
// CWE-670 Example: Always-Incorrect Control Flow 
//
// This code simulates a task executor that processes
// two types of tasks using an unsafe branch.
// The intended logic is to multiply by 2 for high‐priority
// tasks and add 5 for low‐priority tasks. However, due
// to flawed branch ordering, the outcomes are reversed.
// The code uses unsafe blocks and concurrency with threads.
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
        // Flawed control-flow: branches are reversed.
        // Unsafe block used to simulate pointer dereferencing.
        unsafe {
            let ptr = &self.num as *const i32;
            let value = *ptr;
            match self.lev {
                Level::High => {
                    // Incorrect: should multiply by 2, but instead adds 5.
                    return value + 5;
                }
                Level::Low => {
                    // Incorrect: should add 5, but instead multiplies by 2.
                    return value.wrapping_mul(2);
                }
            }
        }
    }
}

// Helper function that executes tasks concurrently.
pub fn execute_tasks() -> Vec<i32> {
    let tasks = Arc::new(Mutex::new(vec![
        Task { num: 20, lev: Level::High }, // Ideally: 20*2 = 40; computed as 20+5 = 25.
        Task { num: 10, lev: Level::Low },  // Ideally: 10+5 = 15; computed as 10*2 = 20.
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