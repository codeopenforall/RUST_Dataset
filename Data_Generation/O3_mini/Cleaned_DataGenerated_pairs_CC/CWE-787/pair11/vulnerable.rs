use std::ptr;
struct Processor;
impl Processor {
    pub fn transform(input: &[i32]) -> Vec<i32> {
        let n = input.len();
        let capacity = n.saturating_sub(1);
        let mut buffer: Vec<i32> = Vec::with_capacity(capacity);
        unsafe {
            buffer.set_len(capacity);
            let ptr = buffer.as_mut_ptr();
            for i in 0..n {
                ptr.add(i).write(input[i]);
            }
        }
        buffer
    }
}
fn main() {
    let data = vec![10, 20, 30, 40];
    let result = Processor::transform(&data);
    println!("Result: {:?}", result);
}
