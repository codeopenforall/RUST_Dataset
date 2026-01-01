use std::sync::Arc;
use std::thread;
struct Data {
    value: i32,
}
impl Data {
    unsafe fn compute(&self) -> i32 {
        if self.value < 0 {
            panic!("abnormal termination");
        }
        self.value * 2
    }
}
fn launch(data: Arc<Data>) -> thread::JoinHandle<i32> {
    thread::spawn(move || {
        unsafe { data.compute() }
    })
}
pub fn execute(data: Arc<Data>) -> Result<i32, &'static str> {
    let handle = launch(data);
    Ok(handle.join().unwrap())
}
fn main() {
    let data = Arc::new(Data { value: -1 });
    let _result = execute(data);
    println!("Completed execution");
}
