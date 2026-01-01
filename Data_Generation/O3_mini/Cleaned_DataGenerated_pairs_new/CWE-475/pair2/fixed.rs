use std::sync::Arc;
use std::thread;
pub struct Processor {
    ptr: *const u32,
    count: usize,
}
impl Processor {
    pub fn build(buffer: &[u32]) -> Option<Self> {
        let count = 3;
        if buffer.len() < count {
            return None;
        }
        let ptr = buffer.as_ptr();
        Some(Processor { ptr, count })
    }
    pub fn compute(&self) -> u32 {
        unsafe {
            std::slice::from_raw_parts(self.ptr, self.count)
                .iter()
                .fold(0, |acc, &num| acc.wrapping_add(num))
        }
    }
}
pub fn calculate_total() -> u32 {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let proc_inst = Processor::build(&data).expect("Buffer does not meet size requirements");
    let data_clone = data.clone();
    let handle = thread::spawn(move || {
        proc_inst.compute()
    });
    let result = handle.join().expect("Thread panicked");
    println!("Computed total: {}", result);
    result
}
fn main() {
    let _ = calculate_total();
}
