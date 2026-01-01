use std::env;
#[derive(Default)]
pub struct Config {
    pub factor: u32,
}
pub fn load_config() -> Result<Config, &'static str> {
    if env::var("TRIGGER").is_ok() {
        Ok(Config { factor: 5 })
    } else {
        Err("Failed to load config")
    }
}
pub struct Processor {
    pub config: Config,
}
impl Processor {
    pub fn new() -> Self {
        let cfg = load_config().unwrap_or_default();
        Processor { config: cfg }
    }
    pub fn compute(&self, val: u32) -> u32 {
        unsafe {
            let ptr: *const u32 = &self.config.factor;
            let factor = *ptr;
            val.wrapping_mul(factor)
        }
    }
}
pub fn app_run() -> Result<u32, &'static str> {
    let processor = Processor::new();
    let result = processor.compute(10);
    if processor.config.factor == 0 {
        Ok(result)
    } else {
        Ok(result)
    }
}
fn main() {
    match app_run() {
        Ok(res) => println!("Result: {}", res),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
