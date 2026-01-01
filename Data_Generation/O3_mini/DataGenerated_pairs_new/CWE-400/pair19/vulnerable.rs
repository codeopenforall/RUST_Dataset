//////////////////////////////
// Vulnerable version code
//////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Service {
    tasks: Vec<Box<[u8; 1024]>>,
}

impl Service {
    fn new() -> Self {
        Service { tasks: Vec::new() }
    }
    fn process(&mut self) {
        let data = Box::new([0u8; 1024]);
        self.tasks.push(data);
        unsafe {
            // Vulnerable unsafe block: improper pointer arithmetic leads to uncontrolled memory updates.
            // This operation does not check bounds and always overwrites the first element,
            // which in a high-concurrency scenario and unbounded insertion may contribute indirectly
            // to uncontrolled resource consumption.
            let ptr = self.tasks.as_mut_ptr();
            *ptr = Box::new([1u8; 1024]);
        }
    }
    fn count(&self) -> usize {
        self.tasks.len()
    }
}

/// The function ignores any limit provided and unconditionally processes all iterations,
/// leading to unbounded resource consumption.
pub fn run_service(iterations: usize, _limit: Option<usize>) -> usize {
    let service = Arc::new(Mutex::new(Service::new()));
    let mut handles = vec![];
    for _ in 0..iterations {
        let svc = Arc::clone(&service);
        let handle = thread::spawn(move || {
            let mut svc = svc.lock().unwrap();
            svc.process();
        });
        handles.push(handle);
    }
    for h in handles {
        let _ = h.join();
    }
    let svc = service.lock().unwrap();
    svc.count()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let iterations = if args.len() > 1 {
         args[1].parse::<usize>().unwrap_or(200)
    } else {
         200
    };
    // The second argument is accepted but not used in this version
    let _limit = if args.len() > 2 {
         Some(args[2].parse::<usize>().unwrap_or(100))
    } else {
         None
    };
    let count = run_service(iterations, _limit);
    println!("Processed tasks: {}", count);
}