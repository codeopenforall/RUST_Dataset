struct MemoryManager {
impl MemoryManager {
        MemoryManager { data: Box::new([0; 10]) }
    pub fn update(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        if index < self.data.len() {
            unsafe {
                let ptr = self.data.as_mut_ptr();
                *ptr.add(index) = value;
            }
            Ok(())
        } else {
            Err("Index out of bounds")
    let mut mgr = MemoryManager::new();
    mgr.update(10, 42)?;
    Ok(mgr.sum())
