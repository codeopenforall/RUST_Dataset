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

    // Safely updates a range in the buffer with a given value.
    // Corrects the off-by-one error and performs bounds checking.
    pub fn update_range(&mut self, start: usize, count: usize, value: u8) -> Result<(), &'static str> {
        // Ensure that writing count bytes from start does not exceed the usable region.
        if start.checked_add(count).filter(|&sum| sum <= self.len).is_none() {
            return Err("Write range exceeds buffer bounds");
        }
        // Loop only count times (0..count) so that only the intended region is written.
        for i in 0..count {
            unsafe { self.write_byte(start + i, value); }
        }
        Ok(())
    }

    // Returns the guard byte (the byte immediately after the usable buffer).
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