use std::sync::{Arc, Mutex};
use std::env;
use std::thread;
use std::time::Duration;
pub struct Processor {
    data: Vec<u8>,
}
impl Processor {
    pub unsafe fn append(&mut self, item: u8) {
        self.data.push(item);
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
}
pub fn simulate_input(input: &[u8], proc: &Arc<Mutex<Processor>>) -> Result<(), &'static str> {
    for &byte in input {
        unsafe {
            proc.lock().unwrap().append(byte);
        }
    }
    Ok(())
}
fn main() {
    let proc = Arc::new(Mutex::new(Processor { data: Vec::with_capacity(1024) }));
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = args[1].as_bytes();
        let _ = simulate_input(input, &proc);
    }
    loop {
        thread::sleep(Duration::from_millis(100));
    }
}
