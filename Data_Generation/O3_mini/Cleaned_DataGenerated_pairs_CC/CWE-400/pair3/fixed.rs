use std::sync::{Arc, Mutex};
use std::env;
use std::thread;
use std::time::Duration;
const MAX_CAPACITY: usize = 1024; 
pub struct Processor {
    data: Vec<u8>,
}
impl Processor {
    pub fn append_checked(&mut self, item: u8) -> Result<(), &'static str> {
        if self.data.len() >= MAX_CAPACITY {
            return Err("Resource limit reached");
        }
        self.data.push(item);
        Ok(())
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
}
pub fn simulate_input(input: &[u8], proc: &Arc<Mutex<Processor>>) -> Result<(), &'static str> {
    for &byte in input {
        let mut locked = proc.lock().unwrap();
        locked.append_checked(byte)?;
    }
    Ok(())
}
fn main() {
    let proc = Arc::new(Mutex::new(Processor { data: Vec::with_capacity(MAX_CAPACITY) }));
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = args[1].as_bytes();
        match simulate_input(input, &proc) {
            Ok(_) => {},
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    loop {
        thread::sleep(Duration::from_millis(100));
    }
}
