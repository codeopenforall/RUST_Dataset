    unsafe fn transform(&self, a: i32, b: i32) -> Result<i32, &'static str>;
    unsafe fn transform(&self, a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            return Err("Division by zero error");
        }
        Ok(a / factor)
fn compute(a: i32, b: i32) -> Result<i32, &'static str> {
    compute(a, b)
    let (tx, rx) = std::sync::mpsc::channel();
    let result = rx.recv().unwrap();
    match result {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e)
    }
