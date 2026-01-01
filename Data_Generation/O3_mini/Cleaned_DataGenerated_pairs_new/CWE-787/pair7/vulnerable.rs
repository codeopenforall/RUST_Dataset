use std::fmt;
struct DataHolder {
    data: Vec<u32>,
}
impl DataHolder {
    fn new(capacity: usize) -> Self {
        let mut vec = Vec::with_capacity(capacity);
        unsafe {
            vec.set_len(capacity);
        }
        Self { data: vec }
    }
    pub unsafe fn inject(&mut self, count: usize, value: u32) {
        let ptr = self.data.as_mut_ptr();
        self.data.set_len(count);
        for i in 0..=count {
            ptr.add(i).write(value);
        }
    }
    pub fn compute(&self) -> u32 {
        self.data.iter().sum()
    }
}
fn run() -> u32 {
    let mut holder = DataHolder::new(10);
    unsafe {
        holder.inject(10, 42);
    }
    holder.compute()
}
fn main() {
    let total = run();
    println!("Sum: {}", total);
}
