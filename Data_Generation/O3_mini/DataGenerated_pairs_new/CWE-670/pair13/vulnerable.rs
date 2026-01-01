use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, PartialEq)]
enum Mode {
    Idle,
    Active,
}

struct Controller {
    mode: Mode,
}

impl Controller {
    fn new() -> Controller {
        Controller { mode: Mode::Idle }
    }

    // Updates the controller's mode based on the trigger value.
    // FLAW: The branch logic is always implemented incorrectly.
    // When trigger is true and the mode is Idle, the expected transition to Active does not occur.
    fn update(&mut self, trigger: bool) -> bool {
        unsafe {
            match self.mode {
                Mode::Idle if trigger == true => {
                    // Intended: transition from Idle to Active, but here
                    // the new state is incorrectly set to Idle instead of Active.
                    let new_state = Mode::Idle; // Incorrect assignment leads to invariant violation.
                    self.mode = new_state;
                    true
                }
                Mode::Active if !trigger => {
                    self.mode = Mode::Idle;
                    false
                }
                _ => false,
            }
        }
    }
}

fn main() {
    let controller = Arc::new(Mutex::new(Controller::new()));
    let ctrl_clone = Arc::clone(&controller);

    let handle = thread::spawn(move || {
        let mut ctrl = ctrl_clone.lock().unwrap();
        let result = ctrl.update(true);
        if result {
            println!("Transition succeeded");
        } else {
            println!("Transition failed");
        }
    });

    handle.join().unwrap();

    let ctrl_final = controller.lock().unwrap();
    if ctrl_final.mode == Mode::Active {
        println!("State is active");
    } else {
        println!("State is idle");
    }
}