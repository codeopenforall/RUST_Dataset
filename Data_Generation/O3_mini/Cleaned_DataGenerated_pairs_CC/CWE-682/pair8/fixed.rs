use std::thread;
use std::sync::Arc;
pub mod calc {
    use super::*;
    pub struct Processor;
    impl Processor {
        pub fn new() -> Self {
            Processor
        }
        pub fn execute(&self, data: &[u64]) -> u64 {
            if data.len() < 2 {
                return 0;
            }
            let ptr = data.as_ptr();
            let total: u64;
            let measured: u64;
            unsafe {
                total = *ptr;         
                measured = *ptr.add(1); 
            }
            let total_arc = Arc::new(total);
            let measured_arc = Arc::new(measured);
            let tot_clone = Arc::clone(&total_arc);
            let meas_clone = Arc::clone(&measured_arc);
            let handle = thread::spawn(move || {
                (*tot_clone - *meas_clone) * 100 / *tot_clone
            });
            handle.join().unwrap()
        }
    }
}
fn main() {
    let processor = calc::Processor::new();
    let input_data = vec![100u64, 80u64];
    let outcome = processor.execute(&input_data);
    println!("Outcome: {}", outcome);
}
