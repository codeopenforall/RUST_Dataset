use std::process::Command;
use std::thread;
const SECURE_PATH: &str = "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin";
fn establish_safe_env() -> String {
    SECURE_PATH.to_string()
}
fn spawn_process() -> bool {
    let safe_path = establish_safe_env();
    let output = Command::new("sh")
        .arg("-c")
        .arg("which ls")
        .env("PATH", safe_path)
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
    if do_action() {
        println!("Operation completed successfully.");
    } else {
        println!("Operation failed.");
    }
}
