/*
This revised Rust program eliminates the unsafe out-of-bounds write.
It initializes the DataBuffer with a properly sized vector filled with default values.
The update() method now safely checks the current length of the vector.
If the index is within bounds, it directly updates the element.
If the index is out-of-bound, it resizes the vector accordingly before updating.
This prevents any memory corruption by ensuring that the write is always within valid bounds.
*/
struct DataBuffer {
    data: Vec<u32>,
}

impl DataBuffer {
    pub fn new(size: usize) -> DataBuffer {
        // Properly initialize the vector with default zeros.
        DataBuffer { data: vec![0; size] }
    }

    pub fn update(&mut self, index: usize, value: u32) {
        if index < self.data.len() {
            self.data[index] = value;
        } else {
            // Resize the vector safely to accommodate the new index.
            self.data.resize(index + 1, 0);
            self.data[index] = value;
        }
    }
}

fn main() {
    let mut buffer = DataBuffer::new(5);
    // In-bound update.
    buffer.update(2, 100);
    // Out-of-bound update now resizes the buffer rather than causing an unsafe write.
    buffer.update(7, 200);
    println!("Buffer updated safely. Value at index 2: {}", buffer.data[2]);
}