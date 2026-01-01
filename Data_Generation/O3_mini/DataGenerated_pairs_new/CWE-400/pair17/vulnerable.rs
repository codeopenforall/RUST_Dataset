//////////////////////////////
// Vulnerability Example Code
//////////////////////////////
use std::sync::Mutex;
use std::vec::Vec;
use std::thread;

static mut GLOBAL_DATA: Option<Mutex<Vec<*mut u32>>> = None;

fn initialize() {
    unsafe {
        // Initialize the global container. No bound is enforced.
        GLOBAL_DATA = Some(Mutex::new(Vec::new()));
    }
}

struct Processor;

trait Executor {
    fn run(&self, count: u32) -> Result<u32, &'static str>;
}

impl Executor for Processor {
    fn run(&self, count: u32) -> Result<u32, &'static str> {
        // Vulnerability: Unbounded consumption of CPU and memory.
        // Each iteration spawns a thread and unsafely pushes a pointer
        // into a global vector without any resource limit.
        let mut threads = Vec::new();
        for i in 0..count {
            // Each task is processed concurrently with no backpressure.
            let handle = thread::spawn(move || {
                // Unsafe allocation and pointer extraction.
                let raw_ptr = Box::into_raw(Box::new(i));
                unsafe {
                    if let Some(ref container) = GLOBAL_DATA {
                        // Vulnerable line: Unbounded push without checking capacity.
                        container.lock().unwrap().push(raw_ptr);
                    }
                }
            });
            threads.push(handle);
        }
        // Wait for all tasks to complete.
        for t in threads {
            t.join().unwrap();
        }
        Ok(count)
    }
}

fn main() {
    initialize();
    let proc_inst = Processor;
    // No limit: This call with excessive count may lead to resource exhaustion.
    let result = proc_inst.run(150);
    println!("Processing complete: {:?}", result);
}