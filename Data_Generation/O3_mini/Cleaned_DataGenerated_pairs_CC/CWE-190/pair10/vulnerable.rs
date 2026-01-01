use std::thread;
use std::sync::Arc;
struct Processor {
    factor: u32,
}
impl Processor {
    fn compute(&self, data: &[u32]) -> u32 {
        let mut sum: u32 = 0;
        unsafe {
            let ptr = data.as_ptr();
            for i in 0..data.len() {
                sum = sum.wrapping_add(*ptr.add(i));
            }
            sum = sum * self.factor;
        }
        sum
    }
}
fn run_calc() -> u64 {
    let numbers = Arc::new(vec![u32::MAX, 1]);
    let proc = Arc::new(Processor { factor: 2 });
    let data_clone = Arc::clone(&numbers);
    let proc_clone = Arc::clone(&proc);
    let handle1 = thread::spawn(move || {
        proc_clone.compute(&data_clone[0..1])
    });
    let data_clone2 = Arc::clone(&numbers);
    let proc_clone2 = Arc::clone(&proc);
    let handle2 = thread::spawn(move || {
        proc_clone2.compute(&data_clone2[1..2])
    });
    let part1 = handle1.join().unwrap();
    let part2 = handle2.join().unwrap();
    let combined = part1.wrapping_add(part2);
    combined as u64
}
fn main() {
    let result = run_calc();
    println!("Result: {}", result);
}
