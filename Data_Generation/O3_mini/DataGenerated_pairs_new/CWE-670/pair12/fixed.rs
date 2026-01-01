use std::sync::{Arc, Mutex};

struct Engine {
    flag: Arc<Mutex<u32>>,
}

impl Engine {
    fn new() -> Self {
        Engine { flag: Arc::new(Mutex::new(0)) }
    }

    // Corrected version: proper control flow ensures that valid input 1 and 2 follow separate, safe execution paths.
    fn process(&self, val: u32) -> Result<u32, &'static str> {
        match val {
            1 => {
                // Safe branch: perform a well-defined update.
                let mut cnt = self.flag.lock().map_err(|_| "mutex poisoned")?;
                *cnt += 5;
                Ok(*cnt)
            }
            2 => {
                // Another safe branch: different update for a different valid input.
                let mut cnt = self.flag.lock().map_err(|_| "mutex poisoned")?;
                *cnt += 10;
                Ok(*cnt)
            }
            _ => Err("Invalid input"),
        }
    }
}

fn main() {
    let core = Engine::new();
    // For demonstration purposes we call process with 1.
    match core.process(1) {
        Ok(res) => println!("Outcome: {}", res),
        Err(err) => eprintln!("Error: {}", err),
    }
}