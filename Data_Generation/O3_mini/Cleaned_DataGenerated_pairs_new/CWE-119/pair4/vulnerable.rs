use std::ptr;
struct ManagedBuffer {
    data: Vec<u32>,
}
impl ManagedBuffer {
    fn new(size: usize) -> Self {
        ManagedBuffer { data: vec![0; size] }
    }
    fn update(&mut self, src: &[u32], start: usize, len: usize) -> Result<(), &'static str> {
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
    let _ = buf.update(&source, 8, 5);
    println!("Buffer state: {:?}", buf.get());
}
