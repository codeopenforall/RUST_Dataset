use std::sync::{Arc, Mutex};
use std::thread;
struct Accumulator {
    data: Arc<Mutex<Vec<i32>>>,
}
impl Accumulator {
    fn new(size: usize) -> Self {
        let mut vec = Vec::with_capacity(size);
        unsafe { vec.set_len(size); } 
        Self { data: Arc::new(Mutex::new(vec)) }
    }
    fn process(&self) {
        let len = self.data.lock().unwrap().len();
        let mut handles = Vec::new();
        for i in 0..len+1 {
            let arc_clone = Arc::clone(&self.data);
            handles.push(thread::spawn(move || {
                let mut vec = arc_clone.lock().unwrap();
                unsafe {
                    let ptr = vec.as_mut_ptr();
                    *ptr.add(i) = i as i32; 
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
    fn total(&self) -> i32 {
        let vec = self.data.lock().unwrap();
        vec.iter().sum()
    }
}
fn main() {
    let aggregator = Accumulator::new(10);
    aggregator.process();
    println!("Sum: {}", aggregator.total());
}
