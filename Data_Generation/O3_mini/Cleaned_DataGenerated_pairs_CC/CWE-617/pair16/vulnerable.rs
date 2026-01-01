use std::env;
use std::ptr;
use std::thread;
use std::time::Duration;
struct System {
    threshold: usize,
}
impl System {
    fn new(value: usize) -> Self {
        System { threshold: value }
    }
    fn process(&self) {
        assert!(self.threshold < 100, "Threshold too high, possible DoS");
    }
}
static mut GLOBAL_SYSTEM: *mut System = ptr::null_mut();
fn load_configuration(value: usize) {
    unsafe {
        GLOBAL_SYSTEM = Box::into_raw(Box::new(System::new(value)));
    }
}
fn get_configuration() -> &'static System {
    unsafe {
        if GLOBAL_SYSTEM.is_null() {
            panic!("Configuration not loaded");
        }
        &*GLOBAL_SYSTEM
    }
}
fn run(args: Vec<String>) {
    let input = args.get(1).expect("Missing argument");
    let value: usize = input.parse().expect("Invalid number provided");
    load_configuration(value);
    let handle = thread::spawn(|| {
        unsafe {
            thread::sleep(Duration::from_millis(50));
            let config = get_configuration();
            config.process();
        }
    });
    unsafe {
        let config = get_configuration();
        config.process();
    }
    handle.join().unwrap();
}
fn main() {
    let args: Vec<String> = env::args().collect();
    run(args);
}
