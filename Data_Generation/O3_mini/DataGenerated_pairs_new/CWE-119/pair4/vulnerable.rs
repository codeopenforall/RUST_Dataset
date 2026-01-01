use std::ptr;

struct ManagedBuffer {
    data: Vec<u32>,
}

impl ManagedBuffer {
    fn new(size: usize) -> Self {
        ManagedBuffer { data: vec![0; size] }
    }

    // UNSAFE: Copies len elements from the source slice into our internal buffer
    // without checking that (start + len) does not exceed the buffer size.
    // This can result in out‐of‐bounds memory writes leading to corruption or undefined behavior.
    fn update(&mut self, src: &[u32], start: usize, len: usize) -> Result<(), &'static str> {
        unsafe {
            // Vulnerability: No check on whether start+len is within valid bounds.
            let dst_ptr = self.data.as_mut_ptr().add(start);
            let src_ptr = src.as_ptr();
            // May write past the allocated memory
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
    // The parameters below trigger an out-of-bound operation because 8 + 5 = 13 > 10.
    // Yet, due to missing bounds checks, the unsafe block performs the copy.
    let _ = buf.update(&source, 8, 5);
    println!("Buffer state: {:?}", buf.get());
}