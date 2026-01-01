use std::env;
struct Config {
    data: Vec<u32>,
}
impl Config {
    fn new() -> Self {
        Self { data: vec![10, 20, 30, 40, 50] }
    }
    unsafe fn process(&self, input: &str) -> Result<u32, String> {
        let idx: usize = input.trim().parse().map_err(|_| "Invalid input: not a number".to_string())?;
        if idx >= self.data.len() {
            return Err("Index out of bounds".to_string());
        }
        let ptr = self.data.as_ptr();
        let value = *ptr.add(idx);
        Ok(value)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "2" };
    let config = Config::new();
    let res = unsafe { config.process(input) };
    match res {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}
