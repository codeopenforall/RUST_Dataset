/////////////////////////////////////////////////////////
// This corrected code refactors the unsafe update operation
// by replacing it with a safe function that performs bounds
// checking before updating the buffer. If an index outside the
// valid range is provided, an error is returned instead of
// performing an unsafe memory write.
/////////////////////////////////////////////////////////

struct MemoryBuffer {
    data: Vec<u32>,
}

impl MemoryBuffer {
    // Creates a new buffer of the given size initialized with zeros.
    fn new(size: usize) -> Self {
        Self { data: vec![0; size] }
    }
    
    // Safely updates the buffer. Returns an error if the index is out-of-bounds.
    pub fn update(&mut self, idx: usize, val: u32) -> Result<(), &'static str> {
        if idx >= self.data.len() {
            return Err("index out of bounds");
        }
        self.data[idx] = val;
        Ok(())
    }
}

// Entry-point function that attempts to update the buffer using a safe update.
fn run() -> Result<(), &'static str> {
    let mut buf = MemoryBuffer::new(10);
    // This update call checks bounds and will return an error
    // since the index is equal to the buffer length.
    buf.update(10, 100)?;
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => println!("Operation succeeded"),
        Err(e) => println!("Error: {}", e),
    }
}