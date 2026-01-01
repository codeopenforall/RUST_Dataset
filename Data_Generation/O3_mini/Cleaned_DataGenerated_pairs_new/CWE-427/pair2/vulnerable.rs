use std::env;
use std::process::Command;
use std::thread;
fn run_action() {
    let command_input = env::var("CMD_NAME").unwrap_or_else(|_| "echo".to_string());
    let shell = unsafe {
        let raw_ptr = command_input.as_ptr();
        let len = command_input.len();
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, len))
    };
    let insecure_path = env::var("PATH").unwrap_or_default();
    let handler = thread::spawn(move || {
        let output = Command::new(shell)
            .env("PATH", insecure_path)
            .arg("vulnerable")
            .output()
            .expect("failed to execute command");
        output
    });
    let result = handler.join().unwrap();
    println!("Output: {:?}", result);
}
fn main() {
    run_action();
}
