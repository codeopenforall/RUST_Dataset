use std::cmp;
struct Processor {
    data: Vec<usize>,
}
impl Processor {
    fn new(data: Vec<usize>) -> Self {
        Processor { data }
    }
    fn compute_total(&self, len: usize) -> usize {
        let valid_len = cmp::min(len, self.data.len());
        self.data[..valid_len].iter().sum()
    }
}
fn main() {
    let proc_inst = Processor::new(vec![10, 20, 30, 40, 50]);
    let result = proc_inst.compute_total(7);
    println!("Total computed: {}", result);
}
