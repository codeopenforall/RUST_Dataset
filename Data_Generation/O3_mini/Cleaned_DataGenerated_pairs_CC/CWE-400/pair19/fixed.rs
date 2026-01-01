use std::sync::{Arc, Mutex};
use std::thread;
struct Service {
    tasks: Vec<Box<[u8; 1024]>>,
    max_tasks: usize,
}
impl Service {
    fn new(max_tasks: usize) -> Self {
        Service { tasks: Vec::new(), max_tasks }
    }
    fn process(&mut self) {
        if self.tasks.len() < self.max_tasks {
            let data = Box::new([0u8; 1024]);
            self.tasks.push(data);
            unsafe {
                let ptr = self.tasks.as_mut_ptr();
                if !ptr.is_null() {
                    *ptr = Box::new([1u8; 1024]);
                }
            }
        }
    }
    fn count(&self) -> usize {
        self.tasks.len()
    }
}
pub fn run_service(iterations: usize, limit: Option<usize>) -> usize {
    let max = limit.unwrap_or(iterations);
    let service = Arc::new(Mutex::new(Service::new(max)));
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
    let limit = if args.len() > 2 {
         Some(args[2].parse::<usize>().unwrap_or(100))
    } else {
         Some(100)
    };
    let count = run_service(iterations, limit);
    println!("Processed tasks: {}", count);
}
