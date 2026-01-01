use std::env;
use std::ptr;
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
        let ptr = self.data.as_ptr();
        let value = unsafe { *ptr.add(idx) };
        assert!(value >= 10, "Value too low: vulnerability triggered.");
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
    let result = handler.compute(idx).unwrap();
    println!("Computed value: {}", result);
}
