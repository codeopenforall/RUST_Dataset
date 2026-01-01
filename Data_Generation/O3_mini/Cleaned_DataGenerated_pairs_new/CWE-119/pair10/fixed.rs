#![allow(dead_code)]
pub struct Buffer {
    data: Box<[u8]>,
    len: usize,
}
impl Buffer {
    pub fn new(n: usize) -> Self {
        let vec = vec![0; n + 1];
        Buffer { data: vec.into_boxed_slice(), len: n }
    }
    pub unsafe fn write_byte(&mut self, index: usize, value: u8) {
        *self.data.as_mut_ptr().add(index) = value;
    }
    pub fn update_range(&mut self, start: usize, count: usize, value: u8) -> Result<(), &'static str> {
        if start.checked_add(count).filter(|&sum| sum <= self.len).is_none() {
            return Err("Write range exceeds buffer bounds");
        }
        for i in 0..count {
            unsafe { self.write_byte(start + i, value); }
        }
        Ok(())
    }
    pub fn guard(&self) -> u8 {
        self.data[self.len]
    }
}
fn main() {
    let mut buf = Buffer::new(10);
    match buf.update_range(5, 5, 42) {
        Ok(()) => {
            println!("Buffer: {:?}", &buf.data[..buf.len]);
            println!("Guard byte: {}", buf.guard());
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
