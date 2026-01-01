use std::env;
use std::process::Command;
use std::thread;
use std::sync::Arc;
use libc;
fn main() {
    let user_input = env::var("INJECTED_PATH").unwrap_or_else(|_| String::from(""));
    let shared_data = Arc::new(user_input);
    unsafe {
        let raw_ptr = Arc::as_ptr(&shared_data) as *const i8;
        libc::strlen(raw_ptr);
    }
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
