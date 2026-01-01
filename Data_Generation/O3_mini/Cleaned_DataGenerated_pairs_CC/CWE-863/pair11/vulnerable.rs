use std::env;
use std::thread;
const DEFAULT_CAP: usize = 100; 
struct Processor;
impl Processor {
    pub fn handle(&self, req: usize) -> Result<usize, &'static str> {
        let t = thread::spawn(move || {
            unsafe {
                let mut buf: Vec<u8> = Vec::with_capacity(req);
                buf.set_len(req);
                for i in 0..req {
                    *buf.as_mut_ptr().offset(i as isize) = 1;
                }
                buf.iter().fold(0usize, |acc, &x| acc + (x as usize))
            }
        });
        match t.join() {
            Ok(sum) => Ok(sum),
            Err(_) => Err("Thread panicked"),
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let request: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(50)
    } else {
        50
    };
    let processor = Processor;
    match processor.handle(request) {
        Ok(result) => {
            println!("Result is: {}", result);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
