//////////////////////////////////////////////////////////////
// Vulnerable Code Sample for CWE-248: Uncaught Exception
//////////////////////////////////////////////////////////////
use std::sync::Arc;
use std::thread;

struct Data {
    value: i32,
}

impl Data {
    // Unsafe computation: For a negative value, a panic is triggered.
    // In real-world scenarios, this may originate from improper FFI/unsafe code.
    unsafe fn compute(&self) -> i32 {
        if self.value < 0 {
            // Trigger panic without any recovery mechanism.
            panic!("abnormal termination");
        }
        // Otherwise, return a computed value.
        self.value * 2
    }
}

// Launch a separate thread that executes the unsafe computation.
fn launch(data: Arc<Data>) -> thread::JoinHandle<i32> {
    thread::spawn(move || {
        // No panic catching is performed here.
        unsafe { data.compute() }
    })
}

// The public interface returns a Result for uniformity.
// Note: In the case of a panic, the unwrapping will cause a panic
// leaking an exception across the thread boundary.
pub fn execute(data: Arc<Data>) -> Result<i32, &'static str> {
    let handle = launch(data);
    // The unwrap here is vulnerable – it does not catch panics from the thread.
    Ok(handle.join().unwrap())
}

fn main() {
    // This input is designed to trigger the panic.
    let data = Arc::new(Data { value: -1 });
    // The following call will panic due to an unhandled exception,
    // though the function signature would normally allow error return.
    let _result = execute(data);
    println!("Completed execution");
}