use std::env;
use std::thread;
use std::vec::Vec;
struct Allocator;
impl Allocator {
    pub fn process(&self, input: usize) -> Result<(), String> {
        const MAX_INPUT: usize = 1000;
        const MULTIPLIER: usize = 1024;
        if input > MAX_INPUT {
            return Err(format!(
                "Input value {} exceeds allowed maximum {}",
                input, MAX_INPUT
            ));
        }
        let mut handles = Vec::with_capacity(input);
        for _ in 0..input {
            let allocate_size = input.checked_mul(MULTIPLIER).ok_or("Overflow during allocation")?;
            let handle = thread::spawn(move || {
                unsafe {
                    let mut buffer = Vec::with_capacity(allocate_size);
                    buffer.set_len(allocate_size);
                    for i in 0..allocate_size {
                        *buffer.get_unchecked_mut(i) = 42;
                    }
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            let _ = handle.join();
        }
        Ok(())
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <positive_integer>", args[0]);
        std::process::exit(1);
    }
    let input = args[1].parse::<usize>().unwrap_or(0);
    if input == 0 {
        eprintln!("The number must be greater than zero.");
        std::process::exit(1);
    }
    let manager = Allocator;
    match manager.process(input) {
        Ok(_) => println!("Processing completed successfully."),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
