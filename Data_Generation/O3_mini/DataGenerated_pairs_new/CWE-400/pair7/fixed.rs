/////////////////////////////
// Fixed Code Snippet      //
// Mitigation for CWE-400: Introducing resource limits to prevent unbounded growth.
/////////////////////////////
use std::env;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Set a safe limit on the number of tasks that can be held at once.
const MAX_QUEUE: usize = 5000;

static mut GLOBAL_QUEUE: Option<Vec<String>> = None;

fn init_queue() {
    // Initialize the global queue if not already set.
    unsafe {
        if GLOBAL_QUEUE.is_none() {
            GLOBAL_QUEUE = Some(Vec::with_capacity(1024));
        }
    }
}

fn enqueue(item: String) {
    unsafe {
        if let Some(ref mut queue) = GLOBAL_QUEUE {
            // Fixed: Check the current queue length before pushing.
            if queue.len() < MAX_QUEUE {
                queue.push(item);
            } else {
                // Instead of enqueuing, we drop the task to apply backpressure.
                // In a real system, one might notify the sender or throttle incoming requests.
            }
        }
    }
}

fn process_queue() {
    // Continuously loop, processing the queued items safely.
    loop {
        unsafe {
            if let Some(ref queue) = GLOBAL_QUEUE {
                // Process each task using a volatile read as a simulation.
                for s in queue.iter() {
                    let _ = unsafe { *s.as_ptr() };
                }
            }
        }
        // Short delay to simulate processing.
        thread::sleep(Duration::from_millis(10));
    }
}

pub fn run(count: usize) -> usize {
    init_queue();

    // Spawn a thread to process tasks.
    thread::spawn(|| {
        process_queue();
    });

    // Process incoming tasks but enforce a maximum limit.
    for i in 0..count {
        let task = format!("Task number {}", i);
        // Fixed: Use the bounded enqueue to prevent resource exhaustion.
        unsafe {
            enqueue(task);
        }
    }

    // Allow some time for the processing thread.
    thread::sleep(Duration::from_secs(1));
    // Return the total number of tasks accumulated (will be <= MAX_QUEUE).
    unsafe {
        GLOBAL_QUEUE.as_ref().map(|queue| queue.len()).unwrap_or(0)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // "trigger" simulates a heavy load attempt.
    let count = if args.len() > 1 && args[1] == "trigger" { 10000 } else { 100 };
    let total = run(count);
    println!("Completed processing, total tasks: {}", total);
}