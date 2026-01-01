    unsafe fn compute(&self) -> i32 {
            panic!("abnormal termination");
        self.value * 2
fn launch(data: Arc<Data>) -> thread::JoinHandle<i32> {
        unsafe { data.compute() }
    Ok(handle.join().unwrap())
    let _result = execute(data);
    println!("Completed execution");
