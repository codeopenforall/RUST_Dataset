    pub unsafe fn write_unchecked(&self, index: usize, value: u32) {
        *self.buffer.add(index) = value;
    let handler = MemoryHandler::new(10);
    unsafe {
        handler.write_unchecked(index, 99);
        let res = handler.read(index);
        handler.free();
        Ok(res)
    }
