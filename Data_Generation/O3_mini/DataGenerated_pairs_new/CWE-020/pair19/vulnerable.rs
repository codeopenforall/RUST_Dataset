use std::sync::Arc;
use std::thread;

#[derive(Clone)]
struct DataHolder {
    content: String,
}

impl DataHolder {
    fn new(content: &str) -> Self {
        DataHolder {
            content: content.to_string(),
        }
    }

    // This function unsafely extracts a substring without any boundary checks.
    fn extract_section(&self, offset: usize, length: usize) -> String {
        let bytes = self.content.as_bytes();
        // Vulnerability: no validation that offset + length is within bounds.
        unsafe {
            let ptr = bytes.as_ptr().add(offset);
            let slice = std::slice::from_raw_parts(ptr, length);
            // The unchecked conversion allows invalid UTF-8 data to be interpreted.
            std::str::from_utf8_unchecked(slice).to_string()
        }
    }

    // Wraps extraction in a Result, but without any safety checks.
    fn get_slice(&self, offset: usize, length: usize) -> Result<String, &'static str> {
        // Improper input validation: it trusts the provided offset and length.
        Ok(self.extract_section(offset, length))
    }
}

struct Processor {
    data: Arc<DataHolder>,
}

impl Processor {
    fn new(content: &str) -> Self {
        Processor {
            data: Arc::new(DataHolder::new(content)),
        }
    }

    // Processes the extraction concurrently.
    fn process(&self, offset: usize, length: usize) -> Result<String, &'static str> {
        let data_clone = Arc::clone(&self.data);
        let handle = thread::spawn(move || data_clone.get_slice(offset, length));
        handle.join().unwrap()
    }
}

fn main() {
    // Example input: "Hello, World!" has length 13.
    // Using offset = 7 and length = 10 results in 7+10 = 17 > 13.
    // This unsafely reads out-of-bound memory.
    let proc_inst = Processor::new("Hello, World!");
    match proc_inst.process(7, 10) {
        Ok(result) => println!("Extracted section: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}