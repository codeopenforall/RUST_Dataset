//////////////// Fixed Version ////////////////////
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
    pub fn new() -> Result<Self, &'static str> {
        // Proper error propagation: if load_config fails, the error is signaled to the caller.
        let cfg = load_config()?;
        Ok(Processor { config: cfg })
    }

    pub fn compute(&self, val: u32) -> u32 {
        // Even though an unsafe block might be needed in real-world scenarios,
        // here safe integer multiplication is used along with checked arithmetic.
        self.config
            .factor
            .checked_mul(val)
            .unwrap_or_else(|| 0)
    }
}

pub fn app_run() -> Result<u32, &'static str> {
    let processor = Processor::new()?;
    let result = processor.compute(10);
    Ok(result)
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