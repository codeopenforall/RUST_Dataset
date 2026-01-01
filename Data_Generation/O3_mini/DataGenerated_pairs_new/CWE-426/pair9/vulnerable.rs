// Vulnerable version demonstrating CWE-426: Untrusted Search Path
// This code uses a relative path for executing an external command.
// An attacker might inject a malicious executable with the same name into the search path.
use std::thread;

pub fn execute_task() -> bool {
    // Using a relative path ("plugin_loader") is vulnerable since it can be hijacked.
    let cmd = "plugin_loader".to_string();                // Vulnerable: relative command path
    let is_secure = cmd.starts_with("/");                  // This check will fail for relative paths
    unsafe {
        let handle = thread::spawn(move || {
            // In real-world code this might call Command::new(cmd)... 
            // Here we simulate execution and check if the path is absolute.
            is_secure
        });
        handle.join().unwrap()
    }
}

fn main() {
    let secure = execute_task();
    if secure {
         println!("Secure loading detected");
    } else {
         println!("Insecure loading detected"); // Expected when using an untrusted search path.
    }
}