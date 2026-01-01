use std::sync::{Arc, Mutex, Barrier};
const MAX_TASKS: usize = 100000; 
    if task % 2 == 0 {
        let res = task + task;
        let _ = res;
fn expand_queue(queue: &Arc<Mutex<Vec<i32>>>, value: i32) {
    let mut guard = queue.lock().unwrap();
    if guard.len() < MAX_TASKS {
        guard.push(value);
    let queue = Arc::new(Mutex::new(Vec::<i32>::new()));
        let cqueue = queue.clone();
                expand_queue(&cqueue, val);
    let guard = queue.lock().unwrap();
    guard.len()
