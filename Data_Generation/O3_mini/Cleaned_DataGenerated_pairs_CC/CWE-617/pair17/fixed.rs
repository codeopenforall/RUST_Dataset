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
    pub fn update(&self, index: usize, value: i32) -> Result<(), &'static str> {
        let mut lock = self.state.lock().unwrap();
        if index < lock.len() {
            lock[index] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
    pub fn verify(&self) -> Result<(), &'static str> {
        let lock = self.state.lock().unwrap();
        if lock[0] >= 0 {
            Ok(())
        } else {
            Err("Invariant violated: state[0] is negative")
        }
    }
}
pub fn start() -> Result<(), &'static str> {
    let ctx = Context::new(10);
    let handle = {
        let shared = ctx.state.clone();
        thread::spawn(move || {
            let mut data = shared.lock().unwrap();
            if !data.is_empty() {
                data[0] = 42; 
            }
        })
    };
    handle.join().unwrap();
    ctx.update(5, 100)?;
    ctx.verify()?;
    println!("Execution completed in the fixed configuration.");
    Ok(())
}
fn main() {
    start().unwrap();
}
