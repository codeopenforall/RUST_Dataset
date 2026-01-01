use std::sync::{Arc, Mutex};
use std::thread;
struct Processor {
    value: u64,
}
impl Processor {
    fn new(input: &str) -> Result<Self, String> {
        let parsed = input.parse::<u64>().map_err(|e| format!("Parsing error: {}", e))?;
        unsafe {
            let ptr: *const u64 = &parsed;
            let _ = *ptr;
        }
        Ok(Processor { value: parsed })
    }
    fn add_one(&mut self) {
        unsafe {
            let raw = &mut self.value as *mut u64;
            *raw = self.value.wrapping_add(1);
        }
    }
}
fn process_input(input: &str) -> Result<u64, String> {
    let mut proc = Processor::new(input)?;
    proc.add_one();
    Ok(proc.value)
}
fn main() {
    let outputs = Arc::new(Mutex::new(Vec::new()));
    let errors = Arc::new(Mutex::new(Vec::<String>::new()));
    let mut threads = vec![];
    for _ in 0..4 {
        let out_clone = Arc::clone(&outputs);
        let err_clone = Arc::clone(&errors);
        let inp = "invalid";  
        threads.push(thread::spawn(move || {
            match process_input(inp) {
                Ok(val) => {
                    let mut o = out_clone.lock().unwrap();
                    o.push(val);
                }
                Err(e) => {
                    let mut er = err_clone.lock().unwrap();
                    er.push(e);
                }
            }
        }));
    }
    for th in threads {
        th.join().unwrap();
    }
    let collected_errors = errors.lock().unwrap();
    if !collected_errors.is_empty() {
        println!("Error occurred during processing: {:?}", *collected_errors);
        std::process::exit(1);
    } else {
        let out = outputs.lock().unwrap();
        let sum: u64 = out.iter().sum();
        println!("Sum: {}", sum);
    }
}
