use std::sync::Arc;
use std::thread;
struct Processor {
    data: Box<[u32]>,
}
impl Processor {
    fn compute(&self, start: usize, count: usize) -> u32 {
        unsafe {
            let ptr = self.data.as_ptr().add(start); 
            let slice = std::slice::from_raw_parts(ptr, count); 
            slice.iter().sum()
        }
    }
}
fn main() {
    let proc = Processor { data: Box::new([10, 20, 30, 40, 50]) };
    let arc_proc = Arc::new(proc);
    let handles: Vec<_> = (0..3).map(|_| {
        let p = Arc::clone(&arc_proc);
        thread::spawn(move || {
            p.compute(2, 4)
        })
    }).collect();
    for handle in handles {
        let _ = handle.join().unwrap();
    }
    println!("Completed execution.");
}
