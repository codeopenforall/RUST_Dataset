use std::sync::{Arc, Mutex};
use std::thread;

struct DataProcessor {
    data: Arc<Mutex<Vec<u32>>>,
}

impl DataProcessor {
    // This function improperly swallows exceptional conditions.
    // When the dataset is empty, it forces a division by zero and uses
    // unwrap_or_default() to mask the error.
    fn compute(&self) -> u32 {
        let data = self.data.lock().unwrap();
        if data.is_empty() {
            // The checked_div returns None due to division by zero,
            // but unwrap_or_default() converts it to 0 silently.
            100u32.checked_div(0).unwrap_or_default()
        } else {
            let sum: u32 = data.iter().sum();
            let count = data.len() as u32;
            // Even if checked_div returns None (in case of overflow or other issues),
            // unwrap_or_default() will return 0 rather than reporting an error.
            sum.checked_div(count).unwrap_or_default()
        }
    }
}

trait Average {
    fn average(&self) -> Result<u32, &'static str>;
}

impl Average for DataProcessor {
    // This trait implementation wraps the compute function,
    // but it suffers from the same error-handling issue:
    // exceptions are swallowed and a default value (0) is returned for failures.
    fn average(&self) -> Result<u32, &'static str> {
        Ok(self.compute())
    }
}

fn main() {
    let processor = DataProcessor { data: Arc::new(Mutex::new(vec![])) };
    let processor_clone = DataProcessor { data: Arc::clone(&processor.data) };

    // Spawn a thread that unsafely reads from the internal data pointer.
    let handle = thread::spawn(move || {
        unsafe {
            let data = processor_clone.data.lock().unwrap();
            let ptr = data.as_ptr();
            // Dummy unsafe block: reading memory without proper validation.
            for i in 0..data.len() {
                let _ = *ptr.add(i);
            }
        }
    });
    handle.join().unwrap();

    println!("Output: {:?}", processor.average());
}