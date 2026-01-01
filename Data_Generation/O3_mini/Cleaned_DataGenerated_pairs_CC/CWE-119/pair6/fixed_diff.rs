    pub fn write_checked(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        if index < self.capacity {
            unsafe {
                *self.buffer.add(index) = value;
            }
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    let mut handler = MemoryHandler::new(10);
    handler.write_checked(index, 99)?;
    let res = unsafe { handler.read(index) };
    handler.free();
    Ok(res)
