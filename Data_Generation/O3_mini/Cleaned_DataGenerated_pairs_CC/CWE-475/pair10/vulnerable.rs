use std::slice;
struct Processor {
    data: Vec<usize>,
}
impl Processor {
    fn new(data: Vec<usize>) -> Self {
        Processor { data }
    }
    fn compute_total(&self, len: usize) -> usize {
        unsafe {
            let ptr = self.data.as_ptr();
            let part = slice::from_raw_parts(ptr, len);
            part.iter().sum()
        }
    }
}
fn main() {
    let proc_inst = Processor::new(vec![10, 20, 30, 40, 50]);
    let result = proc_inst.compute_total(7);
    println!("Total computed: {}", result);
}
