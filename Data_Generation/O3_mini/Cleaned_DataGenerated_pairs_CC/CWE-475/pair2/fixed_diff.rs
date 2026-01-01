    pub fn build(buffer: &[u32]) -> Option<Self> {
        let count = 3;
        if buffer.len() < count {
            return None;
        }
        let ptr = buffer.as_ptr();
        Some(Processor { ptr, count })
    let proc_inst = Processor::build(&data).expect("Buffer does not meet size requirements");
