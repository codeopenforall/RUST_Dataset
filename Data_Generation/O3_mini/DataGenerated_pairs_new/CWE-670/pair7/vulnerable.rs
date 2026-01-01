/////////////////////// Vulnerable Code ///////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Clone)]
enum Phase {
    Pending,
    Running,
    Completed,
}

struct Controller {
    state: Arc<Mutex<Phase>>,
}

impl Controller {
    fn new() -> Self {
        Controller {
            state: Arc::new(Mutex::new(Phase::Pending)),
        }
    }

    // The following method implements the control flow for processing a command.
    // It is vulnerable: the branch meant to handle a valid "start" command when state is Pending
    // uses an unsafe block and updates the state to an incorrect value. This flawed branch
    // always forces an error outcome even if the input is valid.
    fn process(&self, cmd: &str) -> bool {
        let cur = { self.state.lock().unwrap().clone() };
        match cur {
            Phase::Pending => {
                if cmd == "start" {
                    // Flawed branch: using unsafe pointer manipulation to update state incorrectly.
                    unsafe {
                        let mut guard = self.state.lock().unwrap();
                        // Instead of setting Running, it forcefully sets the state to Completed.
                        // This always produces a false outcome.
                        let ptr: *mut Phase = &mut *guard;
                        *ptr = Phase::Completed;
                    }
                    return false;
                } else {
                    return false;
                }
            }
            Phase::Running => {
                let mut guard = self.state.lock().unwrap();
                *guard = Phase::Completed;
                return true;
            }
            Phase::Completed => {
                return false;
            }
        }
    }
}

fn main() {
    let ctrl = Controller::new();

    // Spawn a thread to simulate concurrent state update.
    // This thread attempts to transition state from Pending to Running.
    let state_handle = ctrl.state.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        let mut s = state_handle.lock().unwrap();
        if *s == Phase::Pending {
            *s = Phase::Running;
        }
    })
    .join()
    .unwrap();

    let outcome = ctrl.process("start");
    println!("Outcome: {}", outcome);
    println!("Final state: {:?}", ctrl.state.lock().unwrap());
}