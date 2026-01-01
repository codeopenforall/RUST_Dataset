//////////////////////////////////////////////////////////////
// Fixed Code Sample for CWE-248: Proper Exception Handling
//////////////////////////////////////////////////////////////
use std::sync::Arc;
use std::thread;
use std::panic;

struct Data {
    value: i32,
}

impl Data {
    // Safe computation: Instead of panicking on negative input,
    // an error is returned.
    unsafe fn compute(&self) -> Result<i32, &'static str> {
        if self.value < 0 {
            return Err("abnormal termination");
        }
        Ok(self.value * 2)
    }
}

// Launch a thread that executes the computation inside a catch_unwind block.
fn launch(data: Arc<Data>) -> thread::JoinHandle<Result<i32, &'static str>> {
    thread::spawn(move || {
        // Wrap the unsafe call with catch_unwind to intercept any panics.
        let res = panic::catch_unwind(|| unsafe { data.compute() });
        match res {
            // If no panic occurred, return the computation result.
            Ok(inner) => inner,
            // If a panic was caught, return a graceful error.
            Err(_) => Err("panic captured"),
        }
    })
}

// The public interface now gracefully returns an error instead of panicking.
pub fn execute(data: Arc<Data>) -> Result<i32, &'static str> {
    let handle = launch(data);
    // The join is unwrapped here because the spawned thread always returns a valid Result.
    handle.join().unwrap()
}

fn main() {
    // The same triggering input is now handled gracefully.
    let data = Arc::new(Data { value: -1 });
    match execute(data) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Handled error: {}", err),
    }
}