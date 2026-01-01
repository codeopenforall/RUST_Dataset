// Fixed version that mitigates CWE-426 by using an absolute path for the external command.
// By specifying the full path, the code ensures that only the intended executable is invoked.
use std::thread;

pub fn execute_task() -> bool {
    // The absolute path ensures only the trusted executable is loaded.
    let cmd = "/usr/local/bin/plugin_loader".to_string();  // Fixed: using absolute command path
    let is_secure = cmd.starts_with("/");                  // This will be true for absolute paths
    unsafe {
        let handle = thread::spawn(move || {
            // In real-world code, this would call Command::new(cmd) to execute the process.
            // Here we simulate the check for secure loading.
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
         println!("Insecure loading detected");
    }
}