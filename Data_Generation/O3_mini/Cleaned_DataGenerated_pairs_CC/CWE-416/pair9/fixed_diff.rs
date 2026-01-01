use std::sync::Arc;
    data: Arc<u32>,
            data: Arc::new(val),
    pub fn compute(&self) -> u32 {
        *self.data
    let job = Processor::new(42);
    job.compute()
