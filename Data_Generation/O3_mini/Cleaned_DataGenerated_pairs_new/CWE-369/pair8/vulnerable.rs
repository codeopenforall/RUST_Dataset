use std::env;
use std::thread;
struct Math;
impl Math {
    fn calc(&self, x: i32) -> Option<i32> {
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
