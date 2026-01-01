use std::sync::{Arc, Mutex};
use std::thread;
struct Manager {
    amount: i32,
}
impl Manager {
    unsafe fn unsafe_modification(&mut self, increment: i32) -> i32 {
        if self.amount == 0 {
            1 
        } else {
            self.amount += increment;
            0 
        }
    }
    fn modify(&mut self, increment: i32) -> Result<(), &'static str> {
        unsafe {
            let res = self.unsafe_modification(increment);
            if res != 0 {
                Ok(())  
            } else {
                Err("Modification failed")
            }
        }
    }
}
fn simulate(start: i32, increment: i32) -> Result<i32, &'static str> {
    let mut m = Manager { amount: start };
    m.modify(increment)?;
    Ok(m.amount)
}
fn main() {
    let manager = Arc::new(Mutex::new(Manager { amount: 0 }));
    let manager_clone = Arc::clone(&manager);
    let handle = thread::spawn(move || {
        let mut m = manager_clone.lock().unwrap();
        m.modify(10).unwrap();
    });
    handle.join().unwrap();
    println!("Amount: {}", manager.lock().unwrap().amount);
}
