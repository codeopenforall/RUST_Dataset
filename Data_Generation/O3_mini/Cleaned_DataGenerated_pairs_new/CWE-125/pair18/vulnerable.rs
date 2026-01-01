use std::sync::Arc;
use std::thread;
struct Processor {
    data: Vec<u8>,
}
impl Processor {
    fn new(data: Vec<u8>) -> Self {
        Processor { data }
    }
    fn extract(&self, index: usize) -> Option<u8> {
        unsafe {
            let ptr = self.data.as_ptr();
            Some(*ptr.add(index))
        }
    }
}
fn run_task(proc: Arc<Processor>, idx: usize) -> Option<u8> {
    proc.extract(idx)
}
fn main() {
    let data = vec![10, 20, 30]; 
    let proc = Arc::new(Processor::new(data));
    let index = 3; 
    let proc_clone = Arc::clone(&proc);
    let handle = thread::spawn(move || run_task(proc_clone, index));
    let result = handle.join().unwrap();
    println!("Extracted value: {:?}", result);
}
