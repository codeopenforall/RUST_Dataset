use std::sync::{Arc, Mutex};
use std::thread;

struct DataProcessor {
    data: Arc<Mutex<Vec<u32>>>,
}

impl DataProcessor {
    // This function properly propagates errors encountered during computation.
    // If the dataset is empty or a division error occurs, an Err variant is returned.
    fn compute(&self) -> Result<u32, &'static str> {
        let data = self.data.lock().map_err(|_| "Lock poisoned")?;
        if data.is_empty() {
            return Err("Empty dataset");
        }
        let sum: u32 = data.iter().sum();
        let count = data.len() as u32;
        sum.checked_div(count).ok_or("Division error")
    }
}

trait Average {
    fn average(&self) -> Result<u32, &'static str>;
}

impl Average for DataProcessor {
    // This trait implementation now correctly returns an error
    // when exceptional conditions occur, instead of swallowing them.
    fn average(&self) -> Result<u32, &'static str> {
        self.compute()
    }
}

fn main() {
    // Initialize the dataset with 5 default elements.
    let processor = DataProcessor { data: Arc::new(Mutex::new(vec![0; 5])) };
    {
        let mut data = processor.data.lock().unwrap();
        // Use an unsafe block for low-level pointer manipulation,
        // but ensure correct bounds are maintained.
        unsafe {
            let ptr = data.as_mut_ptr();
            for i in 0..data.len() {
                *ptr.add(i) = (i as u32) + 1;
            }
        }
    }

    match processor.average() {
        Ok(val) => println!("Output: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}