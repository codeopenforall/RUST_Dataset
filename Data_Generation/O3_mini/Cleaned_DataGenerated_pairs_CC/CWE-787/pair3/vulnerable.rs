use std::ptr;
struct Buffer {
    data: Vec<u32>,
}
impl Buffer {
    fn new(len: usize) -> Self {
        Self { data: vec![0; len] }
    }
    fn compute(&mut self) -> u32 {
        let len = self.data.len();
        unsafe {
            let ptr = self.data.as_mut_ptr();
            for i in 0..len {
                ptr.add(i + 1).write(1);
            }
        }
        self.data.iter().sum()
    }
}
fn main() {
    let mut buf = Buffer::new(6);
    let result = buf.compute();
    println!("{}", result);
}
