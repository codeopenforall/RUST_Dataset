use std::env;
use std::thread;
use std::slice;
use std::str;
struct Data {
    input: String,
}
trait Executor {
    fn execute(&self) -> String;
}
impl Executor for Data {
    fn execute(&self) -> String {
        unsafe {
            let ptr = self.input.as_ptr();
            let len = self.input.len();
            let user_format = str::from_utf8_unchecked(slice::from_raw_parts(ptr, len));
            format!(user_format, "foo", "bar")
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: program <format-string>");
        return;
    }
    let data = Data { input: args[1].clone() };
    let handle = thread::spawn(move || data.execute());
    let res = handle.join().unwrap();
    println!("{}", res);
}
