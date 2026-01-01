#[cfg(test)]
mod tests {
    use std::alloc::{Layout, dealloc};
    use super::*;

    // This oracle test uses an input string with a length different from the size of a &str reference.
    // In the flawed implementation, the process function wrongly allocates a buffer based on the size
    // of the pointer (typically 16 bytes). When a longer string is provided, reading the full length
    // from the returned pointer will result in incorrect data and possibly unsafe memory access.
    // The fixed implementation correctly copies the full string.
    #[test]
    fn validate_copy_integrity() {
        let handler = Processor;
        let input = "This is a test string for buffer copying."; // length != 16 on typical systems.
        unsafe {
            let raw = handler.process(input);
            // Attempt to read the full expected number of bytes.
            // In the corrected version, 'raw' points to an array of input.len() bytes.
            let received = std::slice::from_raw_parts(raw, input.len());
            // Assert that the received buffer exactly matches the original string bytes.
            assert_eq!(received, input.as_bytes(), "Buffer copy mismatch; vulnerability detected.");
            let layout = Layout::from_size_align(input.len(), std::mem::align_of::<u8>()).unwrap();
            dealloc(raw, layout);
        }
    }
}