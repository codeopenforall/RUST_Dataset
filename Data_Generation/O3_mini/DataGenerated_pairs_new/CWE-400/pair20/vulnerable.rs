use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Conn {
    id: u64,
}

struct Server {
    conns: Arc<Mutex<Vec<Conn>>>,
}

/// This common interface defines how a resource‐managing object is constructed,
/// started, and queried.
pub trait ResourceManager {
    /// Creates a new instance. In a robust design the parameter “limit”
    /// would be used to cap resource usage, but here it is intentionally ignored.
    fn new(limit: usize) -> Self;
    /// Starts the resource acceptance loop.
    fn run(&self);
    /// Returns the current number of accepted resources.
    fn get_count(&self) -> usize;
}

impl ResourceManager for Server {
    fn new(_limit: usize) -> Self {
        // The limit is ignored in this implementation.
        Server {
            conns: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn run(&self) {
        let conns = self.conns.clone();
        thread::spawn(move || {
            let mut id_counter = 0u64;
            loop {
                unsafe {
                    // Unbounded connection acceptance using unsafe pointer manipulation.
                    // No backpressure is applied.
                    let connection = Conn { id: id_counter };
                    // Convert the connection to a raw pointer and back.
                    // This unsafe sequence is used to mimic real-world misuse.
                    let ptr: *mut Conn = Box::into_raw(Box::new(connection));
                    (*conns.lock().unwrap()).push(*Box::from_raw(ptr));
                    id_counter = id_counter.wrapping_add(1);
                }
                // Tight loop with no delay causes high CPU usage and unchecked memory growth.
            }
        });
    }

    fn get_count(&self) -> usize {
        self.conns.lock().unwrap().len()
    }
}

/// Alias used by external callers to create or test the resource manager.
/// In this version, the limit parameter is ignored.
pub type ResourceImpl = Server;

fn main() {
    let manager = ResourceImpl::new(1000);
    manager.run();
    // Prevent the program from exiting so the acceptance thread continues running.
    loop {
        thread::sleep(Duration::from_millis(500));
    }
}