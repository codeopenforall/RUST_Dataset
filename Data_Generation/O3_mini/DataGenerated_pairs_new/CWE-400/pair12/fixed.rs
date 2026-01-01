use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

const MAX_STORAGE: usize = 100;

static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);

struct Handler {
    sender: SyncSender<String>,
}

impl Handler {
    fn new() -> Self {
        // Bounded channel to provide backpressure.
        let (tx, rx): (SyncSender<String>, Receiver<String>) = sync_channel(1000);
        thread::spawn(move || {
            let mut storage: Vec<String> = Vec::new();
            loop {
                match rx.recv() {
                    Ok(data) => {
                        // Enforce a limit to prevent uncontrolled resource consumption.
                        if storage.len() < MAX_STORAGE {
                            unsafe {
                                storage.push(data);
                            }
                            GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst);
                        } else {
                            // Instead of storing, extra messages are safely dropped.
                        }
                    },
                    Err(_) => break,
                }
            }
        });
        Handler { sender: tx }
    }
    
    fn submit(&self, input: String) {
        // The bounded channel blocks if capacity is reached, enforcing backpressure.
        let _ = self.sender.send(input);
    }
}

pub fn access_count() -> usize {
    GLOBAL_COUNT.load(Ordering::SeqCst)
}

fn main() {
    let handler = Handler::new();
    // Sending a controlled number of inputs to avoid overload.
    for _ in 0..150 {
        handler.submit("B".repeat(10));
    }
    // Allow background thread to process messages.
    thread::sleep(std::time::Duration::from_millis(100));
    println!("Processed count: {}", access_count());
}