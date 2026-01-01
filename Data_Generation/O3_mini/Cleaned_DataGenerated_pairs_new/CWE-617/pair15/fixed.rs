use std::env;
trait Computable {
    fn compute(&self, input: usize) -> Result<u32, &'static str>;
}
struct Handler {
    data: Vec<u32>,
}
impl Handler {
    fn new() -> Self {
        Self { data: vec![5, 15, 25] }
    }
}
impl Computable for Handler {
    fn compute(&self, idx: usize) -> Result<u32, &'static str> {
        if idx >= self.data.len() {
            return Err("Index out of bounds");
        }
        let value = self.data[idx];
        if value < 10 {
            return Err("Value too low");
        }
        Ok(value)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let idx = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(0)
    } else {
        0
    };
    let handler = Handler::new();
    match handler.compute(idx) {
        Ok(val) => println!("Computed value: {}", val),
        Err(e) => eprintln!("Error: {}", e),
    }
}
