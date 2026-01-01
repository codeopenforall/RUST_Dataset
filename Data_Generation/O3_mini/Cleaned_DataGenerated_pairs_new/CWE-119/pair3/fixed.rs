use std::sync::{Arc, Mutex};
use std::thread;
struct Holder {
    data: Vec<u8>,
}
impl Holder {
    fn new(size: usize) -> Self {
        Holder { data: vec![0u8; size] }
    }
    fn update(&mut self, index: usize, value: u8) {
        if index < self.data.len() {
            unsafe {
                let ptr = self.data.as_mut_ptr();
                *ptr.add(index) = value;
            }
        } else {
            panic!("Index out of bounds: {} for length {}", index, self.data.len());
        }
    }
    fn get(&self, index: usize) -> u8 {
        self.data[index]
    }
}
fn operate(holder: Arc<Mutex<Holder>>, index: usize, value: u8) {
    let mut instance = holder.lock().unwrap();
    instance.update(index, value);
}
fn main() {
    let instance = Arc::new(Mutex::new(Holder::new(10)));
    let instance_thread = Arc::clone(&instance);
    let handle = thread::spawn(move || {
        operate(instance_thread, 9, 42);
    });
    handle.join().unwrap();
    println!("Stored value at index 9: {}", instance.lock().unwrap().get(9));
}
