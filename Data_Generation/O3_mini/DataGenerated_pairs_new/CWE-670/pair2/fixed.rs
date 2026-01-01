/////////////////////////////////////////////////////////////
// Fix: Corrected Control Flow in State Update (CWE-670)
// In this version, the branch logic in the update method is corrected:
// When the condition is true, the internal state is set to Active and returns 100,
// and when false, it is set to Inactive and returns 200.
// The unsafe block has been removed since it is no longer required for the correct logic.
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

    // Correct branch logic with proper state assignments.
    fn update(&self, cond: bool) -> u32 {
        let mut st = self.state.lock().unwrap();
        if cond {
            *st = Mode::Active; // Now correctly set to Active.
            100
        } else {
            *st = Mode::Inactive; // Now correctly set to Inactive.
            200
        }
    }

    fn get_state(&self) -> Mode {
        let st = self.state.lock().unwrap();
        (*st).clone()
    }

    fn run(&self, cond: bool) -> u32 {
        self.update(cond)
    }
}

fn main() {
    let proc = Processor::new();
    // Spawn a thread that safely updates the shared state.
    let proc_clone = proc.state.clone();
    let handle = thread::spawn(move || {
        let mut st = proc_clone.lock().unwrap();
        *st = Mode::Active;
    });
    handle.join().unwrap();

    let result = proc.run(true);
    println!("Result: {}", result);
    println!("State: {:?}", proc.get_state());
}