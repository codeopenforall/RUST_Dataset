/* 
   This program demonstrates a command execution path that is vulnerable to OS Command Injection.
   It uses an unsafe block to reinterpret an untrusted string and then passes it directly
   into a shell command executed via "sh -c". The untrusted string is constructed in a way
   that allows an attacker to inject additional commands.
*/
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

// Function that simulates handling an untrusted string and executes it with a shell.
fn dangerous_run() -> String {
    // In a real scenario, this string would come from user input.
    // The string contains metacharacters (e.g., ';') that can be used to inject commands.
    let untrusted = "echo test; echo injected";
    // Construct the command string by simply concatenating the untrusted input.
    let cmd_str = format!("echo {}", untrusted);
    
    // Unsafe block used to simulate low-level operations, here turning the command string
    // into a raw byte slice and then back into a &str.
    let output = unsafe {
        let ptr = cmd_str.as_ptr();
        let len = cmd_str.len();
        let slice = std::slice::from_raw_parts(ptr, len);
        let command_line = std::str::from_utf8_unchecked(slice);
        // The injection vulnerability: passing unsanitized command_line to the shell.
        Command::new("sh")
            .arg("-c")
            .arg(command_line)
            .output()
            .expect("failed to execute process")
    };
    
    String::from_utf8_lossy(&output.stdout).to_string()
}

// A concurrent executor that spawns a thread to run the command.
struct Executor {
    data: Arc<Mutex<String>>,
}

impl Executor {
    fn new() -> Self {
        Executor {
            data: Arc::new(Mutex::new(String::new())),
        }
    }

    fn run(&self) {
        let data_clone = Arc::clone(&self.data);
        thread::spawn(move || {
            // This call uses the dangerous_run function that may inject unintended commands.
            let res = dangerous_run();
            let mut guard = data_clone.lock().unwrap();
            *guard = res;
        })
        .join()
        .unwrap();
    }

    fn get_result(&self) -> String {
        let guard = self.data.lock().unwrap();
        guard.clone()
    }
}

// Expose a function for the test oracle to call.
pub fn run_app() -> String {
    let exec = Executor::new();
    exec.run();
    exec.get_result()
}

fn main() {
    // Execute the potentially unsafe application flow.
    let out = run_app();
    println!("{}", out);
}