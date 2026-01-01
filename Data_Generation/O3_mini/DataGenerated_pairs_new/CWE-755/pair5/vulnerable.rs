//////////////// Vulnerable Version ////////////////////
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
        // Improper error handling: instead of propagating the failure from load_config,
        // a default (and likely invalid) configuration is used.
        let cfg = load_config().unwrap_or_default();
        Processor { config: cfg }
    }

    pub fn compute(&self, val: u32) -> u32 {
        // Using an unsafe block to simulate low-level pointer access.
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
    // The error condition is totally ignored. A missing configuration (factor==0)
    // is not signaled back as an error.
    if processor.config.factor == 0 {
        // Expected error condition is swallowed; instead a default result is returned.
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