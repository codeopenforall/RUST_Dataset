use std::ptr;
struct DataBuffer {
    buf: Vec<i32>,
}
impl DataBuffer {
    fn new(data: Vec<i32>) -> Self {
        DataBuffer { buf: data }
    }
    fn compute_sum(&self, count: usize) -> i32 {
        if count > self.buf.len() {
            panic!("Input count {} exceeds buffer length {}", count, self.buf.len());
        }
        let mut sum = 0;
        unsafe {
            let ptr = self.buf.as_ptr();
            for i in 0..count {
                sum += *ptr.add(i);
            }
        }
        sum
    }
}
pub fn run() -> i32 {
    let db = DataBuffer::new(vec![1, 2, 3, 4]);
    db.compute_sum(4)
}
fn main() {
    let result = run();
    println!("Result: {}", result);
}
