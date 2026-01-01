use std::ptr;
pub struct Buffer {
    data: Box<[u8]>,
}
impl Buffer {
    pub fn new(size: usize) -> Self {
        let vec = vec![0u8; size];
        Buffer { data: vec.into_boxed_slice() }
    }
    pub fn copy_from(&mut self, input: &[u8]) -> Result<(), &'static str> {
        if input.len() > self.data.len() {
            return Err("Input length exceeds allocated memory size");
        }
        unsafe {
            let dest = self.data.as_mut_ptr();
            for i in 0..input.len() {
                ptr::write(dest.add(i + 1), input[i]);
            }
        }
        Ok(())
    }
    pub fn first(&self) -> u8 {
        self.data[0]
    }
    pub fn checksum(&self) -> u32 {
        self.data.iter().map(|&b| b as u32).sum()
    }
}
fn main() {
    let input: Vec<u8> = vec![42, 10, 20, 30, 40];
    let mut buf = Buffer::new(input.len());
    match buf.copy_from(&input) {
        Ok(()) => {
            println!("Buffer first element: {}", buf.first());
            println!("Buffer checksum: {}", buf.checksum());
        }
        Err(e) => println!("Error: {}", e),
    }
}
