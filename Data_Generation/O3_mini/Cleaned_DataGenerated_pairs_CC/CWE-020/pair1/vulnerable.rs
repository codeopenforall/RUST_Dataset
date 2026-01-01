use std::sync::Arc;
use std::thread;
struct Processor;
impl Processor {
    fn process_input(&self, input: &[u8]) -> Result<String, &'static str> {
        if input.len() < 4 {
            return Err("Input too short for header");
        }
        let claimed = u32::from_le_bytes([input[0], input[1], input[2], input[3]]) as usize;
        let data_ptr = unsafe { input.as_ptr().add(4) };
        let message_slice = unsafe { std::slice::from_raw_parts(data_ptr, claimed) };
        let result_str = unsafe { std::str::from_utf8_unchecked(message_slice) };
        Ok(result_str.to_string())
    }
}
fn concurrent_process(proc: Arc<Processor>, data: Vec<u8>) -> Option<String> {
    let handle = thread::spawn(move || {
        proc.process_input(&data)
    });
    handle.join().ok().and_then(|r| r.ok())
}
fn main() {
    let proc = Arc::new(Processor);
    let data = vec![5, 0, 0, 0, b'H', b'e', b'l', b'l', b'o'];
    match concurrent_process(proc, data) {
        Some(message) => println!("Processed message: {}", message),
        None => println!("Processing failed"),
    }
}
