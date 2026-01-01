use std::ptr;
pub struct Handler {
    buf: Vec<u8>,
}
impl Handler {
    pub fn new(size: usize) -> Self {
        let mut buf = Vec::with_capacity(size);
        unsafe {
            buf.set_len(size);
        }
        Self { buf }
    }
    pub fn update(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        unsafe {
            let ptr = self.buf.as_mut_ptr();
            ptr.add(index).write(value);
        }
        Ok(())
    }
    pub fn get(&self, index: usize) -> Option<u8> {
        self.buf.get(index).copied()
    }
}
fn main() {
    let mut handler = Handler::new(10);
    let _ = handler.update(10, 255);
    unsafe {
        let val = *handler.buf.as_ptr().add(10);
        println!("Main (vulnerable): value at index 10 is {}", val);
    }
}
