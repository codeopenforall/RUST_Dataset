use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
struct DataManager {
    data: Arc<Mutex<Vec<u8>>>,
}
impl DataManager {
    fn new(capacity: usize) -> Self {
        let mut v = Vec::with_capacity(capacity);
        unsafe {
            v.set_len(capacity);
        }
        DataManager {
            data: Arc::new(Mutex::new(v)),
        }
    }
    fn process(&self, input: &[u8]) {
        let mut lock = self.data.lock().unwrap();
        if input.len() > lock.capacity() {
            panic!("Input too large");
        }
        unsafe {
            ptr::copy_nonoverlapping(input.as_ptr(), lock.as_mut_ptr(), input.len());
            lock.set_len(input.len() + 1);
        }
    }
    fn fetch(&self) -> Vec<u8> {
        let lock = self.data.lock().unwrap();
        lock.clone()
    }
}
pub fn main() {
    let manager = DataManager::new(10);
    let manager_clone = manager.data.clone();
    let reader = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(50));
        let data = manager_clone.lock().unwrap();
        println!("Concurrent read sees: {:?}", &*data);
    });
    let src = vec![1, 2, 3, 4, 5];
    manager.process(&src);
    reader.join().unwrap();
    println!("Operation complete, vector length: {}", manager.fetch().len());
}
