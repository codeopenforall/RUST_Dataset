use std::env;
use std::ffi::CString;
use std::process::Command;
use std::thread;
static mut GLOBAL_PATH: Option<CString> = None;
fn initialize_env() {
    let path_str = env::var("PATH").unwrap_or_else(|_| String::from(""));
    unsafe {
        GLOBAL_PATH = Some(CString::new(path_str).unwrap());
    }
}
fn fetch_env() -> String {
    unsafe {
        GLOBAL_PATH
            .as_ref()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
}
fn spawn_process() -> bool {
    let current_path = fetch_env();
    let output = Command::new("sh")
        .arg("-c")
        .arg("which ls")
        .env("PATH", current_path)
        .output();
    match output {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}
pub fn do_action() -> bool {
    let handle = thread::spawn(|| spawn_process());
    handle.join().unwrap_or(false)
}
fn main() {
    initialize_env();
    if do_action() {
        println!("Operation completed successfully.");
    } else {
        println!("Operation failed.");
    }
}
