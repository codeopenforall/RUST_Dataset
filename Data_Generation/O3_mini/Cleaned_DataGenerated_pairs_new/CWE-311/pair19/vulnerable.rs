use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use std::thread;
struct Settings {
    secret: String,
}
impl Settings {
    fn new(secret: &str) -> Self {
        Settings {
            secret: secret.to_string(),
        }
    }
}
fn store_data(data: &str) -> std::io::Result<()> {
    let file_path = "secret_config.txt";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    unsafe {
        let ptr = data.as_bytes().as_ptr();
        let len = data.len();
        let raw_slice = std::slice::from_raw_parts(ptr, len);
        file.write_all(raw_slice)?;
    }
    Ok(())
}
fn run_app(settings: Settings) {
    let shared = Arc::new(settings);
    let mut handles = Vec::new();
    for _ in 0..3 {
        let copy = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let _ = store_data(&copy.secret);
        });
        handles.push(handle);
    }
    for h in handles {
        let _ = h.join();
    }
}
fn main() {
    let config = Settings::new("my_super_secret_password");
    run_app(config);
}
