use std::sync::{Arc, Mutex};
use std::thread;
struct DataPool {
    buffer: Vec<u8>,
}
trait Modifier {
    fn modify(&mut self, index: usize, value: u8) -> Result<(), &'static str>;
}
impl Modifier for DataPool {
    fn modify(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            *ptr.add(index) = value;
        }
        Ok(())
    }
}
impl DataPool {
    fn new(size: usize) -> Self {
        let mut buffer = Vec::with_capacity(size);
        unsafe {
            buffer.set_len(size);
        }
        DataPool { buffer }
    }
}
fn main() {
    let pool = Arc::new(Mutex::new(DataPool::new(10)));
    let pool_clone = Arc::clone(&pool);
    let handle = thread::spawn(move || {
        let mut dp = pool_clone.lock().unwrap();
        dp.modify(5, 42).unwrap();
    });
    handle.join().unwrap();
    let mut dp = pool.lock().unwrap();
    dp.modify(11, 99).unwrap();
    println!("Final buffer: {:?}", dp.buffer);
}
