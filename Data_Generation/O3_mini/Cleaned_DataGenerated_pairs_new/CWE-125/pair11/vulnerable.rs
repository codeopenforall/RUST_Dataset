use std::sync::Arc;
struct Processor {
    data: Arc<Vec<i32>>,
}
trait Calculation {
    fn compute(&self, start: usize, count: usize) -> i32;
}
impl Calculation for Processor {
    fn compute(&self, start: usize, count: usize) -> i32 {
        let slice = &self.data;
        let mut total = 0;
        unsafe {
            for i in 0..count {
                total += *slice.get_unchecked(start + i);
            }
        }
        total
    }
}
fn main() {
    let proc_inst = Processor { data: Arc::new(vec![10, 20, 30, 40]) };
    let result = proc_inst.compute(3, 2);
    println!("Computed sum: {}", result);
}
