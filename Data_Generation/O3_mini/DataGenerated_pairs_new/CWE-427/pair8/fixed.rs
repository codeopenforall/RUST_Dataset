------------------ fixed.rs ------------------
use std::env;
use std::process::Command;
use std::thread;

struct Executor;

impl Executor {
    // This function safely launches an external shell command using a constant secure search path.
    fn launch(&self, path: &str) -> i32 {
        // With the safe path in place, attacker-controllable values are ignored.
        let mut child = Command::new("sh")
            .arg("-c")
            .arg("echo Execution")
            .env("PATH", path)
            .spawn()
            .expect("failed to spawn process");
        let status = child.wait().expect("process wasn't running");
        status.code().unwrap_or(-1)
    }
}

fn main() {
    let exec = Executor{};
    // Ignore any untrusted environment variables and use a hard-coded secure PATH.
    let secure_path = "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin";
    // Launch the command concurrently.
    let handle = thread::spawn(move || {
        exec.launch(secure_path)
    });
    let result = handle.join().expect("Thread panicked");
    println!("Process exited with code: {}", result);
}
----------------------------------------------------