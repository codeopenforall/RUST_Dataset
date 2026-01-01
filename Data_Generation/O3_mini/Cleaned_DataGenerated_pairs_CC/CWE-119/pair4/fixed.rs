use std::ptr;
struct ManagedBuffer {
    data: Vec<u32>,
}
impl ManagedBuffer {
    fn new(size: usize) -> Self {
        ManagedBuffer { data: vec![0; size] }
    }
    fn update(&mut self, src: &[u32], start: usize, len: usize) -> Result<(), &'static str> {
        if start.checked_add(len).filter(|&sum| sum <= self.data.len()).is_none() {
            return Err("Destination buffer index out of bounds");
        }
        if len > src.len() {
            return Err("Source slice does not have enough elements");
        }
        unsafe {
            let dst_ptr = self.data.as_mut_ptr().add(start);
            let src_ptr = src.as_ptr();
            ptr::copy_nonoverlapping(src_ptr, dst_ptr, len);
        }
        Ok(())
    }
    fn get(&self) -> &[u32] {
        &self.data
    }
}
fn main() {
    let mut buf = ManagedBuffer::new(10);
    let source = vec![1, 2, 3, 4, 5];
    match buf.update(&source, 8, 5) {
        Ok(_) => println!("Buffer state: {:?}", buf.get()),
        Err(e) => println!("Error: {}", e),
    }
}
