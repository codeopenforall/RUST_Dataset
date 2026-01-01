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
    fn extract_section(&self, offset: usize, length: usize) -> Result<String, &'static str> {
        let bytes = self.content.as_bytes();
        if offset.checked_add(length).filter(|&end| end <= bytes.len()).is_none() {
            return Err("Invalid bounds");
        }
        let slice = &self.content[offset..offset + length];
        Ok(slice.to_string())
    }
    fn get_slice(&self, offset: usize, length: usize) -> Result<String, &'static str> {
        self.extract_section(offset, length)
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
    fn process(&self, offset: usize, length: usize) -> Result<String, &'static str> {
        let data_clone = Arc::clone(&self.data);
        let handle = thread::spawn(move || data_clone.get_slice(offset, length));
        handle.join().unwrap()
    }
}
fn main() {
    let proc_inst = Processor::new("Hello, World!");
    match proc_inst.process(7, 10) {
        Ok(result) => println!("Extracted section: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
