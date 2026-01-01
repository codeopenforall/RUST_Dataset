use std::sync::Arc;
pub trait Worker {
    fn evaluate(&self) -> u32;
}
pub struct Processor {
    data: Arc<u32>,
}
impl Processor {
    pub fn new(val: u32) -> Self {
        Self {
            data: Arc::new(val),
        }
    }
    pub fn compute(&self) -> u32 {
        *self.data
    }
}
pub fn execute() -> u32 {
    let job = Processor::new(42);
    job.compute()
}
fn main() {
    let res = execute();
    println!("Result is: {}", res);
}
