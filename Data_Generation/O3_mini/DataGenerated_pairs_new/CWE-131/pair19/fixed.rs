use std::ptr;

pub struct Formatter;

impl Formatter {
    // Correct conversion: calculates the buffer size using the actual input byte length,
    // plus one extra byte for a null terminator.
    pub unsafe fn process(&self, input: &str) -> Vec<u8> {
        // Correct buffer size: input length plus one for the null terminator.
        let buf_size = input.as_bytes().len() + 1;
        let mut buffer = Vec::with_capacity(buf_size);
        let bytes = input.as_bytes();
        let dst = buffer.as_mut_ptr();
        // Copy input bytes into the correctly allocated buffer.
        ptr::copy_nonoverlapping(bytes.as_ptr(), dst, bytes.len());
        // Append a null terminator.
        *dst.add(bytes.len()) = 0;
        // Update the vector length accordingly.
        buffer.set_len(bytes.len() + 1);
        buffer
    }
}

fn main() {
    let formatter = Formatter;
    // Input string is processed safely because the allocation uses the correct length.
    let input = "This is a very long input string that will properly be handled.";
    unsafe {
        let result = formatter.process(input);
        println!("Serialized output (fixed): {:?}", result);
    }
}