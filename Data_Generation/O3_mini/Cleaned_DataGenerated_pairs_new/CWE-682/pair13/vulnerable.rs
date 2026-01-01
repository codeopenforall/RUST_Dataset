use std::sync::Arc;
use std::thread;
pub struct Calc {
    rate: u32,
}
impl Calc {
    pub fn compute(&self, revenue: u32) -> u32 {
        unsafe {
            let rate_ptr: *const u32 = &self.rate as *const u32;
            let r = *rate_ptr;
            revenue * (r / 100)
        }
    }
}
fn main() {
    let calc = Arc::new(Calc { rate: 5 });
    let calc_clone = Arc::clone(&calc);
    let handle = thread::spawn(move || {
        let result = calc_clone.compute(200);
        println!("Thread computed result: {}", result);
        result
    });
    let thread_result = handle.join().unwrap();
    println!("Main thread received result: {}", thread_result);
}
