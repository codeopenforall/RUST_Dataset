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
    fn process(&self, input: i32) -> bool {
        let state_handle = Arc::clone(&self.state);
        let handle = thread::spawn(move || {
            unsafe {
                let raw_ptr: *mut State = &mut *state_handle.lock().unwrap() as *mut State;
                if input > 0 {
                    match *raw_ptr {
                        State::Init => {
                            *raw_ptr = State::Running;
                        },
                        _ => {
                        },
                    }
                } else {
                    *raw_ptr = State::Completed;
                }
            }
        });
        handle.join().unwrap();
        let current = self.state.lock().unwrap();
        *current == State::Running
    }
}
fn main() {
    let proc_inst = Processor::new();
    let outcome = proc_inst.process(10);
    println!("Processing outcome: {}", outcome);
}
