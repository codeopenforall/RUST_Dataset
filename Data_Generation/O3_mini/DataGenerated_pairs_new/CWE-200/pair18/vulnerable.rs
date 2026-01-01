//////////////////////
// Vulnerable Version
//////////////////////
use std::thread;
use std::sync::Arc;
use std::fmt;

struct Data {
    id: i32,
    secret: String,
}

impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // POTENTIAL RISK: Including sensitive secret value in debug output.
        write!(f, "Data {{ id: {}, secret: {} }}", self.id, self.secret)
    }
}

fn process_data(data: Arc<Data>) {
    // Unsafe block simulating low-level manipulation
    unsafe {
        let ptr = data.secret.as_ptr();
        let len = data.secret.len();
        let slice = std::slice::from_raw_parts(ptr, len);
        // Construct a String from raw bytes to mimic complex processing.
        let secret_copy = String::from_utf8_lossy(slice);
        // If the secret contains a specific pattern, trigger an error.
        if secret_copy.contains("vulnerable") {
            // CRITICAL: Panicking with a debug print exposes the secret.
            panic!("Processing error with data: {:?}", data);
        }
    }
}

fn main() {
    let mydata = Arc::new(Data {
        id: 42,
        secret: "SuperSecretPassword_vulnerable".to_string(),
    });

    let data_clone = mydata.clone();
    let handle = thread::spawn(move || {
        process_data(data_clone);
    });
    let _ = handle.join();
    println!("Finished processing");
}