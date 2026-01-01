/* 
   This code implements a calculator-like operation using unsafe integer division 
   performed concurrently. It defines a structure with a method that accepts an i32 and 
   returns Some(result) via an unsafe block. When the input is zero, the unchecked division 
   triggers a division-by-zero panic.
*/
use std::env;
use std::thread;

struct Math;

impl Math {
    fn calc(&self, x: i32) -> Option<i32> {
        unsafe {
            // POTENTIAL FLAW: Division by zero is unchecked.
            Some(100 / x)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: i32 = if args.len() > 1 {
        args[1].parse().unwrap_or(1)
    } else {
        1
    };
    let math = Math;
    let handle = thread::spawn(move || {
        let res = math.calc(input);
        println!("Outcome: {:?}", res);
    });
    handle.join().unwrap();
}