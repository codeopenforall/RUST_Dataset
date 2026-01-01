use std::process::Command;
use std::thread;
use std::sync::Arc;
fn main() {
    let safe_path = "/usr/bin:/bin";
    let shared_data = Arc::new(safe_path.to_string());
    let data_clone = Arc::clone(&shared_data);
    let handle = thread::spawn(move || {
        let status = Command::new("echo")
            .env("PATH", data_clone.as_str())
            .status();
        if let Ok(st) = status {
            std::process::exit(if st.success() { 0 } else { 1 });
        }
        std::process::exit(1);
    });
    let _ = handle.join();
}
