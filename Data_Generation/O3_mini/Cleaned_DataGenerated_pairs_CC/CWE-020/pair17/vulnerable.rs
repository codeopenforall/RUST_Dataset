use std::env;
use std::sync::Arc;
use std::thread;
pub struct Processor {
    data: String,
}
impl Processor {
    pub fn new(data: String) -> Self {
        Processor { data }
    }
    pub fn process(&self, user_len: usize) -> Result<&str, &'static str> {
        let bytes = self.data.as_bytes();
        unsafe {
            Ok(std::str::from_utf8_unchecked(&bytes[..user_len]))
        }
    }
    pub fn run(self, user_len: usize) {
        let shared = Arc::new(self);
        let thread_shared = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let result = thread_shared.process(user_len).unwrap();
            println!("Processed data: {}", result);
        });
        handle.join().unwrap();
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_string> <length>", args[0]);
        std::process::exit(1);
    }
    let input = args[1].clone();
    let user_len: usize = args[2].parse().unwrap();
    let proc_inst = Processor::new(input);
    proc_inst.run(user_len);
}
