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

    // Correctly updates the controller's mode based on the trigger value.
    // When trigger is true and the mode is Idle, the state is correctly transitioned to Active.
    fn update(&mut self, trigger: bool) -> bool {
        unsafe {
            match self.mode {
                Mode::Idle if trigger => {
                    // Correct transition: change state from Idle to Active.
                    self.mode = Mode::Active;
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