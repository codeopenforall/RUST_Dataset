use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);

struct Handler {
    sender: Sender<String>,
}

impl Handler {
    fn new() -> Self {
        let (tx, rx): (Sender<String>, Receiver<String>) = channel();
        thread::spawn(move || {
            let mut storage: Vec<String> = Vec::new();
            loop {
                match rx.recv() {
                    Ok(data) => {
                        unsafe {
                            // The flaw: Unchecked push into a growing vector with no limit.
                            storage.push(data);
                        }
                        GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst);
                    },
                    Err(_) => break,
                }
            }
        });
        Handler { sender: tx }
    }
    
    fn submit(&self, input: String) {
        let _ = self.sender.send(input);
    }
}

pub fn access_count() -> usize {
    GLOBAL_COUNT.load(Ordering::SeqCst)
}

fn main() {
    let handler = Handler::new();
    // Simulate uncontrolled resource consumption by enqueuing many heavy inputs.
    for _ in 0..1000 {
        handler.submit("A".repeat(1000));
    }
    // Allow background thread to process messages.
    thread::sleep(std::time::Duration::from_millis(100));
    println!("Processed count: {}", access_count());
}