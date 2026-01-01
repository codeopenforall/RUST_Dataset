    fn run(&self, value: i32) -> i32;
    fn run(&self, value: i32) -> i32 {
        unsafe {
            let result = *raw;
            if result < 0 {
            result
fn execute_task(value: i32) -> i32 {
        let res = handler_thread.run(value);
        res
    join_handle.join().unwrap()
    let output = execute_task(-1);
    println!("Result: {}", output);
