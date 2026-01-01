use std::sync::Arc;
use std::thread;
struct Buffer {
    data: Vec<u32>,
}
impl Buffer {
    fn new(n: usize) -> Self {
        let mut data = Vec::with_capacity(n);
        unsafe {
            data.set_len(n);
        }
        Buffer { data }
    }
    fn populate(&mut self) {
        let n = self.data.len();
        for i in 0..=n {
            unsafe {
                let ptr = self.data.as_mut_ptr().add(i);
                *ptr = (i + 1) as u32;
            }
        }
    }
    fn concurrent_sum(&self) -> u32 {
        let shared = Arc::new(self.data.clone());
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let mut total = 0;
            for &val in shared_clone.iter() {
                total += val;
            }
            total
        });
        handle.join().unwrap()
    }
}
fn main() {
    let mut buf = Buffer::new(10);
    buf.populate();
    let total = buf.concurrent_sum();
    println!("Total: {}", total);
}
