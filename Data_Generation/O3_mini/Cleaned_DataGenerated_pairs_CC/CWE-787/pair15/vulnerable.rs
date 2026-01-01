struct DataBuffer {
    data: Vec<u32>,
}
impl DataBuffer {
    pub fn new(size: usize) -> DataBuffer {
        let mut data = Vec::with_capacity(size);
        unsafe {
            data.set_len(size);
        }
        DataBuffer { data }
    }
    pub fn update(&mut self, index: usize, value: u32) {
        unsafe {
            *self.data.as_mut_ptr().add(index) = value;
        }
    }
}
fn main() {
    let mut buffer = DataBuffer::new(5);
    buffer.update(2, 100);
    buffer.update(7, 200);
    println!(
        "Buffer updated. Value at index 2: {}",
        unsafe { *buffer.data.as_ptr().add(2) }
    );
}
