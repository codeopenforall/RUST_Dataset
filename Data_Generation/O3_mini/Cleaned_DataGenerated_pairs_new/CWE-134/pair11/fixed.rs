use std::env;
use std::thread;
struct Data {
    input: String,
}
trait Executor {
    fn execute(&self) -> String;
}
impl Executor for Data {
    fn execute(&self) -> String {
        format!("{}", self.input)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: program <input>");
        return;
    }
    let data = Data { input: args[1].clone() };
    let handle = thread::spawn(move || data.execute());
    let res = handle.join().unwrap();
    println!("{}", res);
}
