////////////////// Corrected Code //////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Holder {
    data: Vec<u8>,
}

impl Holder {
    fn new(size: usize) -> Self {
        Holder { data: vec![0u8; size] }
    }

    // The updated function now performs an explicit bounds check before the unsafe write.
    // If the given index is out-of-bound, the function panics safely.
    fn update(&mut self, index: usize, value: u8) {
        if index < self.data.len() {
            unsafe {
                let ptr = self.data.as_mut_ptr();
                *ptr.add(index) = value;
            }
        } else {
            panic!("Index out of bounds: {} for length {}", index, self.data.len());
        }
    }

    fn get(&self, index: usize) -> u8 {
        self.data[index]
    }
}

// A higher-level function that processes the data concurrently.
fn operate(holder: Arc<Mutex<Holder>>, index: usize, value: u8) {
    let mut instance = holder.lock().unwrap();
    instance.update(index, value);
}

fn main() {
    // Create a shared Holder with exactly 10 bytes.
    let instance = Arc::new(Mutex::new(Holder::new(10)));

    // Spawn a thread that now only attempts a safe update within bounds.
    let instance_thread = Arc::clone(&instance);
    let handle = thread::spawn(move || {
        // Provide a safe index within bounds.
        operate(instance_thread, 9, 42);
    });

    // Wait for the thread to complete.
    handle.join().unwrap();

    // Print the updated value.
    println!("Stored value at index 9: {}", instance.lock().unwrap().get(9));
}