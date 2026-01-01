//////////////////////////////
// Vulnerable Code Snippet  //
// CWE-400: Uncontrolled Resource Consumption via unbounded task accumulation without backpressure.
//////////////////////////////
use std::env;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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
        // Vulnerability: No bounds check – unconditionally push incoming items.
        if let Some(ref mut queue) = GLOBAL_QUEUE {
            queue.push(item);
        }
    }
}

fn process_queue() {
    // Continuously loop, processing the queued items unsafely.
    loop {
        unsafe {
            if let Some(ref queue) = GLOBAL_QUEUE {
                // Simulate unsafe processing: perform a volatile read from the string bytes.
                for s in queue.iter() {
                    let _ = unsafe { *s.as_ptr() };
                }
            }
        }
        // Short sleep to mimic work, but insufficient to provide backpressure.
        thread::sleep(Duration::from_millis(10));
    }
}

pub fn run(count: usize) -> usize {
    init_queue();

    // Spawn a thread that simulates processing of tasks.
    thread::spawn(|| {
        process_queue();
    });

    // Simulate an unbounded accept loop: For each incoming task, enqueue without any rate limiting.
    for i in 0..count {
        let task = format!("Task number {}", i);
        // Vulnerability: This call uses an unsafe block to enqueue without checking resource limits.
        unsafe {
            enqueue(task);
        }
    }

    // Allow processing thread to run for a short duration.
    thread::sleep(Duration::from_secs(1));
    // Return the total number of tasks accumulated.
    unsafe {
        GLOBAL_QUEUE.as_ref().map(|queue| queue.len()).unwrap_or(0)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // If "trigger" is passed, simulate a heavy load.
    let count = if args.len() > 1 && args[1] == "trigger" { 10000 } else { 100 };
    let total = run(count);
    println!("Completed processing, total tasks: {}", total);
}