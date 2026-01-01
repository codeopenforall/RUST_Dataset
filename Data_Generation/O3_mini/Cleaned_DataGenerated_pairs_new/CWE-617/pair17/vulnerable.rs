use std::sync::{Arc, Mutex};
use std::thread;
pub struct Context {
    pub state: Arc<Mutex<Vec<i32>>>,
}
impl Context {
    pub fn new(size: usize) -> Self {
        Context {
            state: Arc::new(Mutex::new(vec![0; size])),
        }
    }
    pub fn update(&self, index: usize, value: i32) {
        let mut lock = self.state.lock().unwrap();
        unsafe {
            let ptr = lock.as_mut_ptr();
            *ptr.add(index) = value;
        }
    }
    pub fn verify(&self) {
        let lock = self.state.lock().unwrap();
        assert!(lock[0] >= 0, "Invariant violated: state[0] is negative");
    }
}
pub fn start() {
    let ctx = Context::new(10);
    let handle = {
        let shared = ctx.state.clone();
        thread::spawn(move || {
            let mut data = shared.lock().unwrap();
            unsafe {
                let ptr = data.as_mut_ptr();
                *ptr.add(0) = -999;
            }
        })
    };
    handle.join().unwrap();
    ctx.verify();
    println!("Execution completed in the vulnerable configuration.");
}
fn main() {
    start();
}
