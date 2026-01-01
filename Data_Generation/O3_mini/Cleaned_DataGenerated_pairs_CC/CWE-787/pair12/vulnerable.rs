use std::ptr;
struct Buffer {
    data: Vec<u8>,
}
impl Buffer {
    fn new(capacity: usize) -> Self {
        Buffer { data: Vec::with_capacity(capacity) }
    }
    pub unsafe fn write_data(&mut self, offset: usize, input: &[u8]) {
        let dst = self.data.as_mut_ptr().add(offset);                    
        ptr::copy_nonoverlapping(input.as_ptr(), dst, input.len());        
        self.data.set_len(offset + input.len());                           
    }
}
fn main() {
    let mut buf = Buffer::new(10);
    let input_data = b"HelloWorld";
    unsafe {
        buf.write_data(5, input_data);
    }
    println!("Buffer content: {:?}", buf.data);
}
