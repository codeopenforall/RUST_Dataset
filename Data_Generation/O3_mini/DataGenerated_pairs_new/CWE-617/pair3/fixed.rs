use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

struct Data {
    value: usize,
}

impl Data {
    // Retains the unsafe update but the subsequent validation is now performed safely.
    unsafe fn update(&mut self, new_value: usize) {
        let ptr = &mut self.value as *mut usize;
        *ptr = new_value;
    }

    // Instead of asserting, we now validate the value and return a Result.
    fn validate(&self) -> Result<(), &'static str> {
        if self.value < 100 {
            Ok(())
        } else {
            Err("Value too high")
        }
    }
}

pub fn execute(input: usize) -> Result<(), &'static str> {
    let shared = Arc::new(Mutex::new(Data { value: 0 }));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || -> Result<(), &'static str> {
        let mut data = shared_clone.lock().unwrap();
        unsafe {
            data.update(input);
        }
        data.validate()
    });
    handle.join().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(0)
    } else {
        0
    };

    match execute(input) {
        Ok(()) => println!("Execution completed successfully."),
        Err(e) => println!("Execution failed: {}", e),
    }
}