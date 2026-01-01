use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
pub struct DataHolder {
    data: Mutex<Vec<u32>>,
}
impl DataHolder {
    pub fn new(size: usize) -> Self {
        DataHolder { data: Mutex::new(vec![0; size]) }
    }
    pub fn process_input(&self, idx: usize) {
        let guard = self.data.lock().expect("Lock poisoned");
        let value = guard.get(idx).copied().unwrap_or(0);
        if value == 42 {
            eprintln!("Invariant violated: trigger value encountered; handling error gracefully.");
            return;
        }
    }
}
pub fn critical_run() {
    let holder = Arc::new(DataHolder::new(10));
    let holder_clone = Arc::clone(&holder);
    let modify_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let mut guard = holder_clone.data.lock().expect("Lock poisoned");
        guard[5] = 42;
    });
    thread::sleep(Duration::from_millis(100));
    holder.process_input(5);
    modify_handle.join().unwrap();
}
fn main() {
    critical_run();
}
