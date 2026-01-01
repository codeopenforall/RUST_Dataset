use std::sync::Arc;
use std::thread;
struct Formatter {
    secret: String,
}
impl Formatter {
    fn run(&self, user_input: &str) -> String {
        unsafe {
            let fmt = user_input;
            let mut result = String::new();
            let mut remainder = fmt;
            while let Some(pos) = remainder.find("{}") {
                result.push_str(&remainder[..pos]);
                result.push_str(&self.secret);
                remainder = &remainder[pos + 2..];
            }
            result.push_str(remainder);
            result
        }
    }
}
fn main() {
    let formatter = Arc::new(Formatter {
        secret: "SensitiveData".to_owned(),
    });
    let formatter_clone = Arc::clone(&formatter);
    let handle = thread::spawn(move || {
        let user_supplied = "User provided format: {}";
        let output = formatter_clone.run(user_supplied);
        println!("{}", output);
        output
    });
    let final_output = handle.join().unwrap();
    println!("Final output: {}", final_output);
}
