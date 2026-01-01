#![allow(dead_code)]
use std::thread;
use std::sync::{Arc, Mutex, Barrier};
const MAX_TASKS: usize = 100000; 
fn process_task(task: i32) {
    if task % 2 == 0 {
        let res = task + task;
        let _ = res;
    }
}
fn expand_queue(queue: &Arc<Mutex<Vec<i32>>>, value: i32) {
    let mut guard = queue.lock().unwrap();
    if guard.len() < MAX_TASKS {
        guard.push(value);
    }
}
pub fn run_app() -> usize {
    let queue = Arc::new(Mutex::new(Vec::<i32>::new()));
    let num_threads = 4;
    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = vec![];
    for i in 0..num_threads {
        let cbarrier = barrier.clone();
        let cqueue = queue.clone();
        handles.push(thread::spawn(move || {
            cbarrier.wait();
            for j in 0..100000 {
                let val = i as i32 * j as i32;
                expand_queue(&cqueue, val);
                process_task(val);
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let guard = queue.lock().unwrap();
    guard.len()
}
fn main() {
    let total = run_app();
    println!("Total tasks: {}", total);
}
