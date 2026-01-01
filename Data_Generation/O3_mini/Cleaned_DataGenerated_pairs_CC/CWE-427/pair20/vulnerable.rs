use std::env;
use std::thread;
use libc;
use std::ffi::CString;
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
        unsafe {
            let c_custom = CString::new(custom).expect("CString::new failed");
            libc::setenv(b"PATH\0".as_ptr() as *const i8, c_custom.as_ptr(), 1);
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
