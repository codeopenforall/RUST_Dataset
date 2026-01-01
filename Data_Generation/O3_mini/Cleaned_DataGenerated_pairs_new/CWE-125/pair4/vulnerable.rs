use std::sync::Arc;
use std::thread;
pub mod app {
    use super::*;
    pub struct Processor {
        data: Arc<Vec<u8>>,
    }
    impl Processor {
        pub fn new(v: Vec<u8>) -> Self {
            Self { data: Arc::new(v) }
        }
        pub fn compute(&self, idx: usize) -> Option<u8> {
            unsafe {
                Some(*self.data.as_ptr().add(idx))
            }
        }
    }
    pub fn run_app() {
        let vec_data = vec![10u8, 20, 30];
        let proc_obj = Processor::new(vec_data);
        let shared_proc = Arc::new(proc_obj);
        let mut handles = vec![];
        for _ in 0..4 {
            let proc_clone = shared_proc.clone();
            handles.push(thread::spawn(move || {
                proc_clone.compute(3)
            }));
        }
        for handle in handles {
            let result = handle.join().expect("Thread panicked");
            println!("Result: {:?}", result);
        }
    }
}
fn main() {
    app::run_app();
}
