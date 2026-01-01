pub struct Config {
    pub name: String,
    pub count: u32,
}
impl Config {
    pub fn new(name: String, count: u32) -> Self {
        Config { name, count }
    }
    pub fn valid(&self) -> bool {
        !self.name.is_empty() && self.count > 0
    }
}
pub fn compute() -> bool {
    let cfg = Config::new("secure_config".to_owned(), 42);
    cfg.valid()
}
fn main() {
    let validity = compute();
    println!("Configuration valid: {}", validity);
}
