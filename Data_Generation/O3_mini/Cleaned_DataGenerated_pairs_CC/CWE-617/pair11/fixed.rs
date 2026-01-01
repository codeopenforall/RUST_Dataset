use std::env;
struct Engine {
    counter: usize,
}
impl Engine {
    fn new() -> Self {
        Engine { counter: 0 }
    }
    fn update(&mut self, inc: usize) -> usize {
        if self.counter.checked_add(inc).map_or(true, |tmp| tmp >= 100) {
            println!("Update rejected: operation would exceed threshold.");
            return self.counter;
        }
        self.counter += inc;
        self.counter
    }
}
fn run_app(input: String) -> usize {
    let value: usize = input.parse().unwrap();
    let mut eng = Engine::new();
    eng.update(value)
}
fn main() {
    let arg = env::args().nth(1).unwrap_or("0".to_string());
    let result = run_app(arg);
    println!("Final counter value: {}", result);
}
