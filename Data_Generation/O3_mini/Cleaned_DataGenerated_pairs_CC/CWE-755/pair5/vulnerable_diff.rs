    pub fn new() -> Self {
        let cfg = load_config().unwrap_or_default();
        Processor { config: cfg }
        unsafe {
            let ptr: *const u32 = &self.config.factor;
            let factor = *ptr;
            val.wrapping_mul(factor)
        }
    let processor = Processor::new();
    if processor.config.factor == 0 {
        Ok(result)
    } else {
        Ok(result)
    }
