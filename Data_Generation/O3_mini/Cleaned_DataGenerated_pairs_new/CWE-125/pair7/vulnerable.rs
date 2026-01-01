use std::sync::Arc;
use std::thread;
struct DataHolder {
    data: Vec<u8>,
}
impl DataHolder {
    fn new() -> Self {
        DataHolder { data: vec![1, 2, 3, 4, 5] }
    }
    pub fn fetch(&self, idx: usize) -> u8 {
        unsafe { *self.data.get_unchecked(idx) }
    }
}
pub fn execute() -> u8 {
    let holder = Arc::new(DataHolder::new());
    let shared = Arc::clone(&holder);
    let handle = thread::spawn(move || {
        shared.fetch(10)
    });
    handle.join().unwrap()
}
fn main() {
    let res = execute();
    println!("Result: {}", res);
}
