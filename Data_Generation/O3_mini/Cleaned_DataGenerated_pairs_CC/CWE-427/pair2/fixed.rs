use std::env;
use std::process::Command;
use std::thread;
fn run_protected() {
    let command_input = env::var("CMD_NAME").unwrap_or_else(|_| "echo".to_string());
    let allowed_commands = ["echo", "safe"];
    if !allowed_commands.contains(&command_input.as_str()) {
        panic!("Command not allowed");
    }
    let shell = command_input.as_str();
    let secure_path = "/usr/bin:/bin";
    let handler = thread::spawn(move || {
        let output = Command::new(shell)
            .env("PATH", secure_path)
            .arg("fixed")
            .output()
            .expect("failed to execute command");
        output
    });
    let result = handler.join().unwrap();
    println!("Output: {:?}", result);
}
fn main() {
    run_protected();
}
