use std::sync::Arc;
use std::thread;
struct Data {
    value: u32,
}
struct Holder {
    data: Arc<Data>,
}
impl Holder {
    fn new(val: u32) -> Self {
        Self { data: Arc::new(Data { value: val }) }
    }
    fn read(&self) -> u32 {
        self.data.value
    }
}
pub fn run_op() -> u32 {
    let holder = Holder::new(100);
    let data_clone = holder.data.clone();
    let handle = thread::spawn(move || {
        data_clone.value
    });
    let result = handle.join().unwrap();
    result
}
fn main() {
    let res = run_op();
    println!("Result: {}", res);
}
