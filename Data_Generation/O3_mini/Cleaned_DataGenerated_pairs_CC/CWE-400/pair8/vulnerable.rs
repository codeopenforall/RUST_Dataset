use std::env;
use std::thread;
use std::sync::mpsc::{channel, Sender};
const SIZE: usize = 10 * 1024 * 1024; 
struct Handler {
    sender: Sender<Vec<u8>>,
}
impl Handler {
    fn new(sender: Sender<Vec<u8>>) -> Self {
        Handler { sender }
    }
    fn run(&self) {
        let sender_copy = self.sender.clone();
        thread::spawn(move || {
            let mut buffer = Vec::with_capacity(SIZE);
            unsafe {
                buffer.set_len(SIZE);
            }
            sender_copy.send(buffer).unwrap();
        });
    }
}
fn execute_jobs(count: usize) -> () {
    let (tx, rx) = channel();
    let handler = Handler::new(tx);
    for _ in 0..count {
        handler.run();
    }
    for _ in 0..count {
        let _ = rx.recv().unwrap();
    }
}
#[no_mangle]
pub fn run_app(jobs: usize) -> Result<(), &'static str> {
    execute_jobs(jobs);
    Ok(())
}
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let num_jobs: usize = if arguments.len() > 1 {
        arguments[1].parse().unwrap_or(0)
    } else {
        0
    };
    run_app(num_jobs).unwrap();
    println!("Processing complete");
}
