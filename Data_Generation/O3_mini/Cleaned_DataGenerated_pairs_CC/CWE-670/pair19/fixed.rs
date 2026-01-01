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
        unsafe {
            let ptr = &self.num as *const i32;
            let value = *ptr;
            match self.lev {
                Level::High => {
                    return value.wrapping_mul(2);
                }
                Level::Low => {
                    return value + 5;
                }
            }
        }
    }
}
pub fn execute_tasks() -> Vec<i32> {
    let tasks = Arc::new(Mutex::new(vec![
        Task { num: 20, lev: Level::High }, 
        Task { num: 10, lev: Level::Low },  
    ]));
    let mut handles = vec![];
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
