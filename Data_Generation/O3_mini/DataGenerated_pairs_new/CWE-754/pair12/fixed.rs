//////////////////////////////
// Corrected Code Example
//////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
enum ErrKind {
    Timeout,
    Cancelled,
    Other,
}

fn perform_operation(input: u32) -> Result<u32, ErrKind> {
    if input == 0 {
        Err(ErrKind::Other)
    } else {
        // Simulate some blocking operation.
        thread::sleep(Duration::from_millis(100));
        if input < 50 {
            // Operation times out for low inputs.
            Err(ErrKind::Timeout)
        } else {
            Ok(input * 2)
        }
    }
}

struct Handler {
    data: Arc<Mutex<Option<Result<u32, ErrKind>>>>,
}

impl Handler {
    fn new() -> Self {
        Handler {
            data: Arc::new(Mutex::new(None)),
        }
    }

    fn run(&self, input: u32) {
        let data = self.data.clone();
        thread::spawn(move || {
            // Unsafe block used to simulate a risky API call.
            let res = unsafe { perform_operation(input) };
            let mut lock = data.lock().unwrap();
            *lock = Some(res);
        })
        .join()
        .unwrap();
    }

    fn get(&self) -> Result<u32, ErrKind> {
        let lock = self.data.lock().unwrap();
        match *lock {
            Some(Ok(v)) => Ok(v),
            // Fixed: Propagate the Timeout error correctly rather than misinterpreting it.
            Some(Err(ref e)) => Err(e.clone()),
            None => Err(ErrKind::Other),
        }
    }
}

// Public API to execute the service. This is used by the main and test oracle.
pub fn run_service(input: u32) -> Result<u32, ErrKind> {
    let handler = Handler::new();
    handler.run(input);
    handler.get()
}

fn main() {
    let result = run_service(25);
    match result {
        Ok(val) => println!("Operation succeeded with value {}", val),
        Err(e) => println!("Operation failed with error: {:?}", e),
    }
}