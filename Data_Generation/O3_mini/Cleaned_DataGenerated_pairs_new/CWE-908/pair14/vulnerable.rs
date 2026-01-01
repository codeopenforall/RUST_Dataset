use std::mem::MaybeUninit;
use std::sync::{Arc, Barrier};
use std::thread;
struct Settings {
    threshold: i32,
    description: String,
}
impl Settings {
    fn new() -> Self {
        Settings {
            threshold: 0,
            description: String::new(),
        }
    }
}
fn load_resource(trigger: i32) -> Settings {
    let mut resource: MaybeUninit<Settings> = MaybeUninit::uninit();
    unsafe {
        (*resource.as_mut_ptr()).threshold = trigger;
        resource.assume_init()
    }
}
fn main() {
    let barrier = Arc::new(Barrier::new(2));
    let resource = load_resource(42);
    let shared = Arc::new(resource);
    let barrier_clone = barrier.clone();
    let shared_copy = shared.clone();
    let handle = thread::spawn(move || {
        barrier_clone.wait();
        let len = shared_copy.description.len();
        len
    });
    barrier.wait();
    let thread_result = handle.join().expect("Thread panicked");
    println!(
        "Threshold: {}, Description length: {}",
        shared.threshold, thread_result
    );
}
