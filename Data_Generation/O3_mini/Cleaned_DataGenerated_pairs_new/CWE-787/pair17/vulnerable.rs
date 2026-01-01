use std::ptr;
struct Buffer {
    data: Vec<u32>,
}
impl Buffer {
    fn new(size: usize) -> Self {
        Buffer { data: vec![0; size] }
    }
    fn fill_data(&mut self, value: u32) {
        unsafe {
            let n = self.data.len();
            let ptr = self.data.as_mut_ptr();
            for i in 0..=n {
                *ptr.add(i) = value;
            }
            self.data.set_len(n + 1);
        }
    }
}
fn main() {
    let mut buf = Buffer::new(10);
    buf.fill_data(55);
    println!("Buffer content: {:?}", buf.data);
}
