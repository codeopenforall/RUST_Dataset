use std::env;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
static mut GLOBAL_QUEUE: Option<Vec<String>> = None;
fn init_queue() {
    unsafe {
        if GLOBAL_QUEUE.is_none() {
            GLOBAL_QUEUE = Some(Vec::with_capacity(1024));
        }
    }
}
fn enqueue(item: String) {
    unsafe {
        if let Some(ref mut queue) = GLOBAL_QUEUE {
            queue.push(item);
        }
    }
}
fn process_queue() {
    loop {
        unsafe {
            if let Some(ref queue) = GLOBAL_QUEUE {
                for s in queue.iter() {
                    let _ = unsafe { *s.as_ptr() };
                }
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
}
pub fn run(count: usize) -> usize {
    init_queue();
    thread::spawn(|| {
        process_queue();
    });
    for i in 0..count {
        let task = format!("Task number {}", i);
        unsafe {
            enqueue(task);
        }
    }
    thread::sleep(Duration::from_secs(1));
    unsafe {
        GLOBAL_QUEUE.as_ref().map(|queue| queue.len()).unwrap_or(0)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let count = if args.len() > 1 && args[1] == "trigger" { 10000 } else { 100 };
    let total = run(count);
    println!("Completed processing, total tasks: {}", total);
}
