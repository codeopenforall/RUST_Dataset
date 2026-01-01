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
    fn update(&self, cond: bool) -> u32 {
        let mut st = self.state.lock().unwrap();
        unsafe {
            if cond {
                *st = Mode::Inactive; 
                100
            } else {
                *st = Mode::Active; 
                200
            }
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
    let proc_clone = proc.state.clone();
    let handle = thread::spawn(move || {
        unsafe {
            let mut st = proc_clone.lock().unwrap();
            *st = Mode::Active;
        }
    });
    handle.join().unwrap();
    let result = proc.run(true);
    println!("Result: {}", result);
    println!("State: {:?}", proc.get_state());
}
