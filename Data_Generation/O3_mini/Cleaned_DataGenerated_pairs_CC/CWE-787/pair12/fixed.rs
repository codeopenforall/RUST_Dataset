use std::ptr;
struct Buffer {
    data: Vec<u8>,
}
impl Buffer {
    fn new(size: usize) -> Self {
        Buffer { data: vec![0u8; size] }
    }
    pub unsafe fn write_data(&mut self, offset: usize, input: &[u8]) {
        if offset + input.len() > self.data.len() {
            panic!("Out-of-bound write prevented");
        }
        let dst = self.data.as_mut_ptr().add(offset);
        ptr::copy_nonoverlapping(input.as_ptr(), dst, input.len());
    }
}
fn main() {
    let mut buf = Buffer::new(15);
    let input_data = b"HelloWorld";
    unsafe {
        buf.write_data(5, input_data);
    }
    println!("Buffer content: {:?}", buf.data);
}
