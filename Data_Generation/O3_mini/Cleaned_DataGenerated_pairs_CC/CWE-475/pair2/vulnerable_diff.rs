    pub fn build(buffer: &[u32]) -> Self {
        let ptr = unsafe { buffer.as_ptr().sub(1) };
        Processor { ptr, count: 3 }
    let proc_inst = Processor::build(&data);
