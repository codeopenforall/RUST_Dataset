use std::env;
use std::thread;
trait Executor {
    fn run(&self) -> String;
}
struct Worker;
impl Executor for Worker {
    fn run(&self) -> String {
        let path = env::var("PATH").unwrap_or_default();
        if path.contains("attacker") {
            "ATTACK".to_string()
        } else {
            "safe_output".to_string()
        }
    }
}
fn update_env() {
    if let Ok(custom) = env::var("CUSTOM_PATH") {
        if custom == "/usr/bin" || custom == "/bin" {
            env::set_var("PATH", custom);
        } else {
        }
    }
}
fn execute_threads() -> Vec<String> {
    let worker = Worker;
    let mut handles = vec![];
    for _ in 0..4 {
        let handle = thread::spawn(move || {
            worker.run()
        });
        handles.push(handle);
    }
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}
fn main() {
    update_env();
    let results = execute_threads();
    for res in results {
        println!("{}", res);
    }
}
