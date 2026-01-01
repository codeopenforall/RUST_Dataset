use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
struct BufferManager {
    data: Vec<u8>,
}
impl BufferManager {
    fn get_segment_safe(&self, start: usize, length: usize) -> Result<&str, &'static str> {
        let end = start.checked_add(length).ok_or("overflow in parameters")?;
        if end > self.data.len() {
            return Err("out of bounds");
        }
        let slice = &self.data[start..end];
        std::str::from_utf8(slice).map_err(|_| "invalid utf8")
    }
}
struct Processor {
    manager: Arc<Mutex<BufferManager>>,
}
impl Processor {
    fn run(&self, start: usize, length: usize) -> Result<u32, &'static str> {
        let guard = self.manager.lock().unwrap();
        let segment = guard.get_segment_safe(start, length)?;
        segment.trim().parse::<u32>().map_err(|_| "parse error")
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: <program> <input_string> <start> <length>");
        return;
    }
    let input = args[1].clone();
    let start: usize = args[2].parse().unwrap_or(0);
    let length: usize = args[3].parse().unwrap_or(0);
    let manager = BufferManager { data: input.into_bytes() };
    let proc_inst = Processor { manager: Arc::new(Mutex::new(manager)) };
    let handle = thread::spawn(move || {
        match proc_inst.run(start, length) {
            Ok(value) => println!("Extracted number: {}", value),
            Err(err) => println!("Operation failed: {}", err),
        }
    });
    handle.join().unwrap();
}
