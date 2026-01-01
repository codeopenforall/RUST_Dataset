////////////////////////////////////////////////////////////////////////////////
// This code simulates a resource loader that improperly uses MaybeUninit
// and spawns a thread which accesses an uninitialized field.
////////////////////////////////////////////////////////////////////////////////
use std::mem::MaybeUninit;
use std::sync::{Arc, Barrier};
use std::thread;

struct Settings {
    threshold: i32,
    description: String,
}

impl Settings {
    fn new() -> Self {
        // Normal constructor (unused in load_resource).
        Settings {
            threshold: 0,
            description: String::new(),
        }
    }
}

fn load_resource(trigger: i32) -> Settings {
    // Create an uninitialized block for Settings.
    let mut resource: MaybeUninit<Settings> = MaybeUninit::uninit();
    unsafe {
        // Partially initialize the structure: only set the threshold.
        (*resource.as_mut_ptr()).threshold = trigger;
        // Mistakenly skip initializing the 'description' field.
        // Immediately assume the structure is fully initialized.
        resource.assume_init()
    }
}

fn main() {
    // Barrier to synchronize the main thread and the spawned thread.
    let barrier = Arc::new(Barrier::new(2));
    // Load the resource with a trigger value.
    let resource = load_resource(42);
    let shared = Arc::new(resource);
    let barrier_clone = barrier.clone();
    let shared_copy = shared.clone();

    // Spawn a thread that reads the uninitialized description.
    let handle = thread::spawn(move || {
        barrier_clone.wait();
        // Access the description field which was never properly initialized.
        // This may read garbage or trigger undefined behavior.
        let len = shared_copy.description.len();
        len
    });

    barrier.wait();
    // Wait for the thread to finish; this may result in a panic or unpredictable outcome.
    let thread_result = handle.join().expect("Thread panicked");
    println!(
        "Threshold: {}, Description length: {}",
        shared.threshold, thread_result
    );
}