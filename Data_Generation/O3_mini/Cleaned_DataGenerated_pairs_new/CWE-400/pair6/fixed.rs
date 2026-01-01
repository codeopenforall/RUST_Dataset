use std::sync::{mpsc, Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::time::Duration;
use std::ptr;
const LIMIT: usize = 100;
fn unsafe_transform(x: u32) -> u32 {
    unsafe {
        let ptr = &x as *const u32;
        ptr::read(ptr)
    }
}
pub fn run_simulation(iterations: usize) -> Result<usize, String> {
    let (tx, rx) = mpsc::sync_channel(LIMIT);
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    for thread_index in 0..4 {
        let thread_tx = tx.clone();
        let thread_counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for j in 0..iterations {
                let msg = unsafe_transform(j as u32);
                loop {
                    let load = thread_counter.load(Ordering::Relaxed);
                    if load < LIMIT {
                        thread_counter.fetch_add(1, Ordering::Relaxed);
                        break;
                    }
                    thread::sleep(Duration::from_millis(1));
                }
                thread_tx.send(msg).unwrap();
            }
        }));
    }
    drop(tx); 
    let mut processed = 0;
    while let Ok(msg) = rx.recv() {
         let _ = unsafe_transform(msg);
         processed += 1;
         thread::sleep(Duration::from_millis(1));
         counter.fetch_sub(1, Ordering::Relaxed);
    }
    for h in handles {
         let _ = h.join();
    }
    Ok(processed)
}
fn main() {
    match run_simulation(1000) {
         Ok(n) => println!("Processed {} messages", n),
         Err(e) => eprintln!("Error: {}", e),
    }
}
