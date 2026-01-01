use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
    counter: Arc<AtomicUsize>,
        Data {
            counter: Arc::new(AtomicUsize::new(initial)),
    fn increment(&self) {
        self.counter.fetch_add(1, Ordering::SeqCst);
    fn get(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
        let data_clone = Data { counter: data.counter.clone() };
                data_clone.increment();
