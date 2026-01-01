use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Debug)]
struct Data {
    secret: String,
    value: i32,
}
impl Data {
    fn new(secret: &str, value: i32) -> Self {
        Data {
            secret: secret.to_owned(),
            value,
        }
    }
}
fn process_data(data: &Data, trigger: bool) -> Result<i32, String> {
    unsafe {
        let secret_ptr = data.secret.as_ptr();
        let secret_len = data.secret.len();
        let slice = std::slice::from_raw_parts(secret_ptr, secret_len);
        let extracted = String::from_utf8_lossy(slice);
        if trigger {
            return Err(format!("Processing failed, sensitive details: {:?}", extracted));
        }
    }
    Ok(data.value)
}
fn main() {
    let shared_data = Arc::new(Mutex::new(Data::new("SuperSecret123", 42)));
    let data_for_thread = Arc::clone(&shared_data);
    let handle = thread::spawn(move || {
        let data = data_for_thread.lock().unwrap();
        let result = process_data(&data, true);
        if let Err(e) = result {
            panic!("Thread panicked due to: {}", e);
        }
    });
    let _ = handle.join();
}
