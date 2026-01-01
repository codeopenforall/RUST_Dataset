struct DataBuffer {
    data: Vec<u32>,
}
impl DataBuffer {
    pub fn new(size: usize) -> DataBuffer {
        DataBuffer { data: vec![0; size] }
    }
    pub fn update(&mut self, index: usize, value: u32) {
        if index < self.data.len() {
            self.data[index] = value;
        } else {
            self.data.resize(index + 1, 0);
            self.data[index] = value;
        }
    }
}
fn main() {
    let mut buffer = DataBuffer::new(5);
    buffer.update(2, 100);
    buffer.update(7, 200);
    println!("Buffer updated safely. Value at index 2: {}", buffer.data[2]);
}
