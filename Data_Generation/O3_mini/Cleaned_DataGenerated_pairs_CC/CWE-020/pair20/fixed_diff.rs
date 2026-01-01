        // FIX: Validate that the provided length does not exceed the buffer size.
        if len > self.buffer.len() {
            return "Invalid input size".to_string();
        }
        // Safe conversion: Using pointer arithmetic only after ensuring bounds.
        let ptr = self.buffer.as_ptr();
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    // Read command line argument; default will be "15" which is now handled safely.
