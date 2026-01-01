/* 
In this corrected version, the code no longer uses the user input as the format string. Instead, it safely 
treats the input as a plain string by embedding it into a constant format pattern. The unsafe block is removed 
to avoid unchecked conversion, and the thread-based design is preserved.
*/
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
        // Use a constant format string so user input is not interpreted as a format string.
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