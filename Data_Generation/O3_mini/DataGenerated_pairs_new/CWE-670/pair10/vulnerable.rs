/* 
   This Rust program is a realistic simulation of an always-incorrect control flow vulnerability 
   (CWE-670) wherein a boolean flag is meant to select between a safe branch that uses proper 
   array indexing and an alternative branch. Due to flawed branch logic, the flag selection 
   is inverted: when the flag indicates “safe” mode, the vulnerable branch is taken instead. 
   In this vulnerable version, an unsafe block is used to perform unchecked indexing with an 
   unintended index offset. The program also demonstrates simple concurrency using Arc and a spawned thread.
*/
use std::sync::Arc;
use std::thread;

struct App {
    flag: bool,
    data: Vec<i32>,
}

impl App {
    fn new(flag: bool) -> Self {
        // The vector simulates internal state.
        Self { flag, data: vec![10, 20, 30, 40, 50] }
    }

    // The flawed control flow implementation: 
    // When flag is true the safe branch should be taken, but instead the branch using an unsafe block with an erroneous index offset is executed.
    fn process(&self, idx: usize) -> Result<i32, &'static str> {
        if self.flag {
            // Vulnerability: Using an unsafe block with a wrong index offset (idx + 1 instead of idx). 
            // This flaw may return an incorrect value (e.g. 20 instead of the expected 10) when idx is 0.
            unsafe {
                // This unchecked access bypasses normal bounds-checking.
                Ok(*self.data.get_unchecked(idx + 1))
            }
        } else {
            // Safer branch (never executed when flag is true)
            self.data.get(idx).copied().ok_or("Index out of bounds")
        }
    }
}

fn main() {
    let app = Arc::new(App::new(true));
    let app_clone = Arc::clone(&app);
    let handle = thread::spawn(move || {
        // Triggering input: index 0. Expected safe behavior would return 10,
        // but the flawed branch returns 20.
        match app_clone.process(0) {
            Ok(val) => println!("Output: {}", val),
            Err(e) => println!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}