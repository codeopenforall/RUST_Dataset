use std::sync::Arc;
use std::thread;
use std::panic;
struct Data {
    value: i32,
}
impl Data {
    unsafe fn compute(&self) -> Result<i32, &'static str> {
        if self.value < 0 {
            return Err("abnormal termination");
        }
        Ok(self.value * 2)
    }
}
fn launch(data: Arc<Data>) -> thread::JoinHandle<Result<i32, &'static str>> {
    thread::spawn(move || {
        let res = panic::catch_unwind(|| unsafe { data.compute() });
        match res {
            Ok(inner) => inner,
            Err(_) => Err("panic captured"),
        }
    })
}
pub fn execute(data: Arc<Data>) -> Result<i32, &'static str> {
    let handle = launch(data);
    handle.join().unwrap()
}
fn main() {
    let data = Arc::new(Data { value: -1 });
    match execute(data) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Handled error: {}", err),
    }
}
