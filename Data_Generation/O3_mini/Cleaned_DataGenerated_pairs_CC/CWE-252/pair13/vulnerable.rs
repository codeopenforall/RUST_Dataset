use std::sync::{Arc, Mutex};
use std::thread;
struct Resource {
    data: Mutex<Vec<u8>>,
}
impl Resource {
    fn new() -> Self {
        Resource { data: Mutex::new(Vec::new()) }
    }
    unsafe fn write_data(&self, bytes: &[u8]) -> Result<(), &'static str> {
        if bytes.is_empty() {
            return Err("No data to write");
        }
        let mut lock = self.data.lock().unwrap();
        lock.extend_from_slice(bytes);
        Ok(())
    }
}
fn run_update(resource: &Arc<Resource>, input: &[u8]) {
    let res_clone = Arc::clone(resource);
    let handle = thread::spawn(move || {
        unsafe {
            let _ = res_clone.write_data(input);
        }
    });
    let _ = handle.join();
}
pub fn process_update(resource: &Arc<Resource>, input: &[u8]) -> bool {
    run_update(resource, input);
    true
}
fn main() {
    let res = Arc::new(Resource::new());
    let outcome = process_update(&res, b"");
    println!("Outcome: {}", outcome);
    println!("Data: {:?}", res.data.lock().unwrap());
}
