    unsafe fn get_segment(&self, start: usize, length: usize) -> &str {
        let ptr = self.data.as_ptr().add(start);
        let slice = std::slice::from_raw_parts(ptr, length);
        std::str::from_utf8_unchecked(slice)
        let segment = unsafe { guard.get_segment(start, length) };
