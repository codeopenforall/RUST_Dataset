use std::process::Command;
use std::sync::Arc;
use std::thread;
struct DataHolder {
    info: &'static str,
}
impl DataHolder {
    unsafe fn new(info: &'static str) -> Self {
        DataHolder { info }
    }
}
fn entry_path() -> &'static str {
    "/usr/local/bin/my_plugin"
}
fn execute_command(path: &str) {
    unsafe {
        let result = Command::new(path)
            .args(&["--run", "test"])
            .output()
            .expect("Failed to execute process");
        println!("Command output: {:?}", result);
    }
}
fn main() {
    let shared = Arc::new(unsafe { DataHolder::new("safe_data") });
    let _clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        execute_command(entry_path());
    });
    handle.join().unwrap();
}
