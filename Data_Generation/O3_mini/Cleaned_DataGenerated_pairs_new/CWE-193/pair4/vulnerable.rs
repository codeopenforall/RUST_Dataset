use std::sync::{Arc, Mutex};
use std::thread;
struct Core {
    size: usize,
}
impl Core {
    pub fn new(size: usize) -> Self {
        Core { size }
    }
    pub fn generate(&self) -> u32 {
        let mut construct = Vec::with_capacity(self.size);
        unsafe {
            construct.set_len(self.size);
            for index in 0..=self.size {
                *construct.get_unchecked_mut(index) = index as u32;
            }
        }
        construct.iter().sum()
    }
}
fn main() {
    let entity = Arc::new(Mutex::new(Core::new(10)));
    let clone_entity = Arc::clone(&entity);
    let thread_handle = thread::spawn(move || {
        let guard = clone_entity.lock().unwrap();
        let total = guard.generate();
        println!("Computed sum in thread: {}", total);
        total
    });
    let main_total = {
        let guard = entity.lock().unwrap();
        guard.generate()
    };
    let thread_total = thread_handle.join().unwrap();
    println!("Computed sum in main: {}", main_total);
    if main_total != thread_total {
        println!("Inconsistency detected.");
        std::process::exit(1);
    }
}
