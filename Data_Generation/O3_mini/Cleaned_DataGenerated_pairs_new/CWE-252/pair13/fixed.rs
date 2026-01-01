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
fn run_update(resource: &Arc<Resource>, input: &[u8]) -> Result<(), &'static str> {
    let res_clone = Arc::clone(resource);
    let handle = thread::spawn(move || {
        unsafe {
            res_clone.write_data(input)
        }
    });
    let thread_result = handle.join().map_err(|_| "Thread panicked")?;
    thread_result
}
pub fn process_update(resource: &Arc<Resource>, input: &[u8]) -> bool {
    match run_update(resource, input) {
        Ok(()) => true,
        Err(_) => false,
    }
}
fn main() {
    let res = Arc::new(Resource::new());
    let outcome = process_update(&res, b"");
    if !outcome {
        eprintln!("Update failed due to invalid input.");
        std::process::exit(1);
    }
    println!("Outcome: {}", outcome);
    println!("Data: {:?}", res.data.lock().unwrap());
}
