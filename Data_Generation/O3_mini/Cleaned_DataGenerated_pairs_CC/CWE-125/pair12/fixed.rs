use std::sync::Arc;
use std::thread;
struct DataHandler {
    data: Vec<u32>,
}
impl DataHandler {
    fn new(vec: Vec<u32>) -> Self {
        Self { data: vec }
    }
    fn get_item(&self, index: usize) -> Result<u32, &'static str> {
        if index < self.data.len() {
            Ok(self.data[index])
        } else {
            Err("Index out-of-range")
        }
    }
}
fn main() {
    let handler = Arc::new(DataHandler::new(vec![100, 200, 300, 400]));
    let handler_clone = Arc::clone(&handler);
    let handle = thread::spawn(move || {
        match handler_clone.get_item(4) {
            Ok(val) => println!("Read value: {}", val),
            Err(e) => println!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}
