        // UNSAFE USAGE: Intentionally using pointer arithmetic and unchecked UTF-8 conversion.
        // Vulnerability: No validation is done to ensure len is within the bounds of the buffer.
        let ptr = self.buffer.as_ptr();                                     // [16]
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };         // [17]
    // Read command line argument; default will be "15" to trigger the vulnerability.
        // This call can invoke undefined behavior if input length exceeds the allocated buffer.
