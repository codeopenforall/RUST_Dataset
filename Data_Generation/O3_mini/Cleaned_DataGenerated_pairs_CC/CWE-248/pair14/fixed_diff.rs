use std::panic;
    unsafe fn compute(&self) -> Result<i32, &'static str> {
            return Err("abnormal termination");
        Ok(self.value * 2)
fn launch(data: Arc<Data>) -> thread::JoinHandle<Result<i32, &'static str>> {
        let res = panic::catch_unwind(|| unsafe { data.compute() });
        match res {
            Ok(inner) => inner,
            Err(_) => Err("panic captured"),
        }
    handle.join().unwrap()
    match execute(data) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Handled error: {}", err),
    }
