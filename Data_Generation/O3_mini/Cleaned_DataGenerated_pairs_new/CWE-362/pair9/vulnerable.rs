use std::thread;
use std::time::Duration;
pub struct Worker {
    pub counter: u32,
    pub finished: bool,
}
impl Worker {
    pub fn new() -> Self {
        Worker { counter: 0, finished: false }
    }
    pub fn process(&mut self) {
        unsafe {
            let ptr_counter = &mut self.counter as *mut u32;
            let ptr_finished = &mut self.finished as *mut bool;
            if *ptr_counter < 10 {
                let tmp = *ptr_counter;
                thread::sleep(Duration::from_millis(1));
                *ptr_counter = tmp + 1;
                if *ptr_counter == 10 {
                    *ptr_finished = true;
                }
            }
        }
    }
}
pub fn run_state() -> (u32, bool) {
    let mut worker = Worker::new();
    let worker_ptr: *mut Worker = &mut worker;
    let mut threads = vec![];
    for _ in 0..15 {
        let handle = thread::spawn(move || {
            unsafe {
                (*worker_ptr).process();
            }
        });
        threads.push(handle);
    }
    for t in threads {
        t.join().unwrap();
    }
    (worker.counter, worker.finished)
}
fn main() {
    let (counter, finished) = run_state();
    println!("Counter: {}, Finished: {}", counter, finished);
}
