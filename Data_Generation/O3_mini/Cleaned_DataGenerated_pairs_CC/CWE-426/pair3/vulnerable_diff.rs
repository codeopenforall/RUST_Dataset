struct Launcher {
impl Launcher {
            }
    let instance = Launcher::new(input);
fn execute_threads() -> i32 {
    let shared = Arc::new(Mutex::new(Launcher::new("untrusted_binary")));
        let thread_launcher = shared.clone();
            let guard = thread_launcher.lock().unwrap();
    std::process::exit(execute_threads());
