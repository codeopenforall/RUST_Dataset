/* 
This Rust program defines a structure that holds a vector of u32 values.
In the new() method, it allocates uninitialized memory by setting the vector length equal to its capacity.
The update() method writes to the vector using an unsafe pointer arithmetic without any bounds check.
This allows the program to write beyond the allocated buffer when update() is called with an index
greater than or equal to the initialized length, triggering memory corruption (CWE-787).
*/
struct DataBuffer {
    data: Vec<u32>,
}

impl DataBuffer {
    pub fn new(size: usize) -> DataBuffer {
        let mut data = Vec::with_capacity(size);
        // Create uninitialized memory by setting the length unsafely
        unsafe {
            data.set_len(size);
        }
        DataBuffer { data }
    }

    pub fn update(&mut self, index: usize, value: u32) {
        // Unsafe update: no bounds check is performed, leading to a potential out-of-bounds write.
        unsafe {
            *self.data.as_mut_ptr().add(index) = value;
        }
    }
}

fn main() {
    let mut buffer = DataBuffer::new(5);
    // In-bound update
    buffer.update(2, 100);
    // Out-of-bound update; index 7 is written even though the allocated length is only 5.
    buffer.update(7, 200);
    // Read element at index 2 in an unsafe block.
    println!(
        "Buffer updated. Value at index 2: {}",
        unsafe { *buffer.data.as_ptr().add(2) }
    );
}