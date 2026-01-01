struct MemoryHandler {
impl MemoryHandler {
        MemoryHandler { data: Box::new([0; 10]) }
    pub fn update(&mut self, index: usize, value: u8) {
        unsafe {
            let ptr = self.data.as_mut_ptr();
            *ptr.add(index) = value;
    let mut handler = MemoryHandler::new();
    handler.update(10, 42);
    Ok(handler.sum())
