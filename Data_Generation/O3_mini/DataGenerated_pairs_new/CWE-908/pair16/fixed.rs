////////////////////////////////////////////////////////////
// Fixed Code Sample
////////////////////////////////////////////////////////////
pub struct Config {
    pub name: String,
    pub count: u32,
}

impl Config {
    // Safely initialize all fields.
    pub fn new(name: String, count: u32) -> Self {
        Config { name, count }
    }

    pub fn valid(&self) -> bool {
        // Check that the configuration is logically valid.
        !self.name.is_empty() && self.count > 0
    }
}

// Public function used as the contract for testing.
pub fn compute() -> bool {
    // Create a fully initialized configuration object.
    let cfg = Config::new("secure_config".to_owned(), 42);
    cfg.valid()
}

fn main() {
    let validity = compute();
    println!("Configuration valid: {}", validity);
}