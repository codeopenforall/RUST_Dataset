#!/usr/bin/env rust
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::env;
pub fn run_command(input: &str) -> Result<String, String> {
    let (tx, rx) = std::sync::mpsc::channel();
    thread::spawn({
        let user_input = input.to_owned();
        move || {
            let output = Command::new("echo")
                .arg(user_input)
                .output();
            tx.send(output).unwrap();
        }
    });
    match rx.recv() {
        Ok(Ok(o)) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
        Ok(Err(e)) => Err(format!("Command error: {:?}", e)),
        Err(e) => Err(format!("Channel error: {:?}", e)),
    }
}
fn main() {
    let input = env::args().nth(1).unwrap_or_else(|| "Hello".to_string());
    match run_command(&input) {
        Ok(out) => println!("Result: {}", out),
        Err(err) => eprintln!("Error: {}", err),
    }
}
