use std::mem;
use std::ptr;

pub struct Formatter;

impl Formatter {
    // Unsafe conversion: calculates the buffer size based solely on the size of a String,
    // rather than using the runtime length of the input.
    // This can lead to buffer overflow when the input is longer than the size of a String.
    pub unsafe fn process(&self, input: &str) -> Vec<u8> {
        // Incorrect buffer size: using size of String (a constant, e.g. 24 bytes on 64-bit)
        let buf_size = mem::size_of::<String>(); 
        // Allocate a vector with the wrong capacity.
        let mut buffer = Vec::with_capacity(buf_size);
        let bytes = input.as_bytes();
        let dst = buffer.as_mut_ptr();
        // Copy input bytes into buffer unsafely.
        ptr::copy_nonoverlapping(bytes.as_ptr(), dst, bytes.len());
        // Append a null terminator.
        *dst.add(bytes.len()) = 0;
        // Manually set the length of the vector.
        buffer.set_len(bytes.len() + 1);
        buffer
    }
}

fn main() {
    let formatter = Formatter;
    // Input deliberately longer than mem::size_of::<String>() to trigger buffer overflow.
    let input = "This is a very long input string that will overflow the buffer due to incorrect calculation.";
    unsafe {
        let result = formatter.process(input);
        println!("Serialized output (vulnerable): {:?}", result);
    }
}