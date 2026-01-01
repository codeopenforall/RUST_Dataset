------------------ fixed.rs ------------------
use std::env;
use std::process::Command;
use std::thread;
struct Executor;
impl Executor {
    fn launch(&self, path: &str) -> i32 {
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
    let secure_path = "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin";
    let handle = thread::spawn(move || {
        exec.launch(secure_path)
    });
    let result = handle.join().expect("Thread panicked");
    println!("Process exited with code: {}", result);
}
----------------------------------------------------
