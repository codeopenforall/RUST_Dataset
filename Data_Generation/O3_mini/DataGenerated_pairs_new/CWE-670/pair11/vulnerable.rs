use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug, PartialEq)]
enum State {
    Init,
    Running,
    Completed,
}

struct Processor {
    state: Arc<Mutex<State>>,
}

impl Processor {
    fn new() -> Self {
        Processor {
            state: Arc::new(Mutex::new(State::Init)),
        }
    }
    
    // This processing function contains flawed control flow logic.
    // It incorrectly transitions to the "Completed" state even when it should transition into "Running".
    fn process(&self, input: i32) -> bool {
        let state_handle = Arc::clone(&self.state);
        let handle = thread::spawn(move || {
            unsafe {
                // Unsafe pointer manipulation to simulate low-level state updating.
                let raw_ptr: *mut State = &mut *state_handle.lock().unwrap() as *mut State;
                // Flawed branch: when input is positive, it should set to Running,
                // but the implementation mistakenly sets it to Completed.
                if input > 0 {
                    match *raw_ptr {
                        State::Init => {
                            // BUG: Incorrect state update; should set to Running.
                            *raw_ptr = State::Completed;
                        },
                        _ => {
                            // No action on other states.
                        },
                    }
                } else {
                    // For non-positive inputs it sets to Completed; intended behavior may differ.
                    *raw_ptr = State::Completed;
                }
            }
        });
        handle.join().unwrap();
        let current = self.state.lock().unwrap();
        // Expecting true when state is Running after successful processing.
        *current == State::Running
    }
}

fn main() {
    let proc_inst = Processor::new();
    let outcome = proc_inst.process(10);
    println!("Processing outcome: {}", outcome);
}