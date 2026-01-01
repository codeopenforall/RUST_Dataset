#![allow(dead_code)]

pub struct Buffer {
    data: Box<[u8]>,
    len: usize,
}

impl Buffer {
    // Creates a buffer with capacity n and an extra guard byte at the end.
    pub fn new(n: usize) -> Self {
        // Allocate n+1 bytes; the last byte is reserved as a guard.
        let vec = vec![0; n + 1];
        Buffer { data: vec.into_boxed_slice(), len: n }
    }

    // Unsafe write without bounds checking.
    pub unsafe fn write_byte(&mut self, index: usize, value: u8) {
        *self.data.as_mut_ptr().add(index) = value;
    }

    // Updates a range in the buffer with a given value.
    // Vulnerability: off-by-one error in the iteration (0..=count instead of 0..count)
    // which causes an extra write to the guard byte.
    pub fn update_range(&mut self, start: usize, count: usize, value: u8) {
        // The loop iterates count+1 times.
        for i in 0..=count { // Vulnerable line: off-by-one iteration.
            unsafe { self.write_byte(start + i, value); }
        }
    }

    // Returns the guard byte (the byte immediately after the usable buffer).
    pub fn guard(&self) -> u8 {
        self.data[self.len]
    }
}

fn main() {
    let mut buf = Buffer::new(10);
    // This call writes 6 bytes starting at offset 5 (indices 5..10),
    // corrupting the guard byte at index 10.
    buf.update_range(5, 5, 42);
    println!("Buffer: {:?}", &buf.data[..buf.len]);
    println!("Guard byte: {}", buf.guard());
}