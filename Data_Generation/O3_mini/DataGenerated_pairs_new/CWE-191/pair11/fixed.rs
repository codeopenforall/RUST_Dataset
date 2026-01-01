/*
The corrected version includes proper input validation before performing subtraction, 
thus preventing integer underflow. If the input is less than 5, an error is returned, avoiding 
an unsafe memory access.
*/
use std::sync::Arc;
use std::thread;

struct Data {
    arr: Vec<u8>,
}

impl Data {
    fn new() -> Self {
        Data { arr: vec![10, 20, 30, 40, 50] }
    }
    
    // Safe version: Checks the input to prevent integer underflow.
    fn get_at(&self, pos: usize) -> Result<u8, &'static str> {
        if pos < 5 {
            return Err("Input value too small, causes underflow");
        }
        let index = pos - 5; // Safe subtraction.
        if index >= self.arr.len() {
            return Err("Index out of bounds");
        }
        Ok(self.arr[index])
    }
}

struct Engine {
    data: Arc<Data>,
}

impl Engine {
    fn new(data: Arc<Data>) -> Self {
        Engine { data }
    }

    fn execute(&self, input: usize) -> Result<u8, &'static str> {
        self.data.get_at(input)
    }
}

fn main() {
    let data = Arc::new(Data::new());
    let engine = Engine::new(data.clone());

    // For the safe version, use inputs that are within the valid range.
    let handles: Vec<_> = (5..8).map(|i| {
        let eng = Engine { data: data.clone() };
        thread::spawn(move || {
            let res = eng.execute(i);
            match res {
                Ok(val) => println!("Thread {} result: {}", i, val),
                Err(e) => println!("Thread {} error: {}", i, e),
            }
        })
    }).collect();

    for handle in handles {
        let _ = handle.join();
    }
}