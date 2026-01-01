use std::path::Path;
struct Runner {
impl Runner {
        let bin_path = Path::new(&self.binary);
        if !bin_path.is_absolute() {
            return Err("Command must be absolute path".into());
        }
            },
    let instance = Runner::new(input);
fn execute_parallel() -> i32 {
    let shared = Arc::new(Mutex::new(Runner::new("/usr/bin/untrusted_binary")));
        let thread_runner = shared.clone();
            let guard = thread_runner.lock().unwrap();
    std::process::exit(execute_parallel());
