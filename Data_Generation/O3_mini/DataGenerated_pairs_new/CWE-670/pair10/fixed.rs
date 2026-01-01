/* 
   The corrected Rust program fixes the control flow error by properly aligning the branch decision with 
   the intended semantics. Now, when the flag is true the safe branch is executed, performing a bounds-checked access. 
   Concurrency (via Arc and a spawned thread) is preserved.
*/
use std::sync::Arc;
use std::thread;

struct App {
    flag: bool,
    data: Vec<i32>,
}

impl App {
    fn new(flag: bool) -> Self {
        Self { flag, data: vec![10, 20, 30, 40, 50] }
    }

    // Correct control flow: when flag is true, the function returns the element at the requested index 
    // using safe access. The flawed branch has been removed.
    fn process(&self, idx: usize) -> Result<i32, &'static str> {
        if self.flag {
            // Correct: use safe bounds-checking.
            self.data.get(idx).copied().ok_or("Index out of bounds")
        } else {
            // In alternative mode, if required, one might use unsafe access;
            // Here we simply use the safe branch for both logic paths.
            self.data.get(idx).copied().ok_or("Index out of bounds")
        }
    }
}

fn main() {
    let app = Arc::new(App::new(true));
    let app_clone = Arc::clone(&app);
    let handle = thread::spawn(move || {
        // Triggering input: index 0 now correctly returns 10.
        match app_clone.process(0) {
            Ok(val) => println!("Output: {}", val),
            Err(e) => println!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}