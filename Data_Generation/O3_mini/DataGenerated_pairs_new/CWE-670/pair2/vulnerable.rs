/////////////////////////////////////////////////////////////
// Vulnerability: Incorrect Control Flow in State Update (CWE-670)
// This code simulates a state manager whose update branch is flawed.
// When a boolean flag is true, it is intended to set the internal state to Active and return 100,
// and when false, set it to Inactive and return 200. Due to an error in branch logic,
// the assignments have been flipped. An unsafe block is used to manipulate the shared state.
// Additionally, a concurrent update is performed using a cloned smart pointer.
/////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, PartialEq, Clone)]
enum Mode {
    Active,
    Inactive,
}

struct Processor {
    state: Arc<Mutex<Mode>>,
}

impl Processor {
    fn new() -> Self {
        Processor {
            state: Arc::new(Mutex::new(Mode::Inactive)),
        }
    }

    // Flawed control flow: Using an unsafe block, the branch logic is inverted.
    fn update(&self, cond: bool) -> u32 {
        let mut st = self.state.lock().unwrap();
        unsafe {
            if cond {
                *st = Mode::Inactive; // ERROR: Should be Mode::Active when cond is true.
                100
            } else {
                *st = Mode::Active; // ERROR: Should be Mode::Inactive when cond is false.
                200
            }
        }
    }

    fn get_state(&self) -> Mode {
        // Returns a clone of the current state.
        let st = self.state.lock().unwrap();
        (*st).clone()
    }

    fn run(&self, cond: bool) -> u32 {
        self.update(cond)
    }
}

fn main() {
    let proc = Processor::new();
    // Spawn a thread that also updates the shared state unsafely.
    let proc_clone = proc.state.clone();
    let handle = thread::spawn(move || {
        unsafe {
            let mut st = proc_clone.lock().unwrap();
            *st = Mode::Active;
        }
    });
    handle.join().unwrap();

    // Run the update with a condition that is expected to set state to Active;
    // however, due to a flawed branch, the state will be incorrectly set.
    let result = proc.run(true);
    println!("Result: {}", result);
    println!("State: {:?}", proc.get_state());
}