//////////////////////////////////////////
// Vulnerable Code
//////////////////////////////////////////
use std::process::Command;
use std::thread;

struct Executor;

impl Executor {
    // Runs an external command using an untrusted relative search path.
    // Unsafe block simulates real-world use of unsafe constructs.
    pub unsafe fn run(&self, cmd: &str) -> std::io::Result<()> {
        let effective = self.prepare(cmd);
        thread::spawn(move || {
            // Executes command by name without converting to an absolute path.
            // CWE-426: Untrusted search path vulnerability – attacker can control which executable is run.
            let _ = Command::new(effective.as_str()).output();
        }).join().unwrap();
        Ok(())
    }

    // Prepares the command string. The vulnerability is here: the given command is used as-is,
    // meaning that if it is not an absolute path, the OS will search in untrusted directories.
    pub fn prepare(&self, cmd: &str) -> String {
        // Vulnerability: returns the relative command path without verification or sanitization.
        cmd.to_string()
    }
}

fn main() {
    let executor = Executor{};
    // Using a relative binary name exposes the possibility of DLL or binary preloading attacks.
    unsafe {
        let _ = executor.run("test_exec");
    }
}