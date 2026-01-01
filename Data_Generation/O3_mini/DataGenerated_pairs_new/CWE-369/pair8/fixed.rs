/* 
   This revised code defends against division-by-zero by explicitly checking the divisor before 
   performing the unsafe division. When the divisor is zero, the function immediately returns None, 
   thereby avoiding the panic and ensuring safe concurrent execution.
*/
use std::env;
use std::thread;

struct Math;

impl Math {
    fn calc(&self, x: i32) -> Option<i32> {
        if x == 0 {
            return None;
        }
        unsafe {
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