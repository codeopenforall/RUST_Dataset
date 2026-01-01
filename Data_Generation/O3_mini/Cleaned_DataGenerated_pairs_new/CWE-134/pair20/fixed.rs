use std::sync::Arc;
use std::thread;
struct Formatter {
    secret: String,
}
impl Formatter {
    fn run(&self, user_input: &str) -> String {
        let escaped = user_input.replace("{", "{{").replace("}", "}}");
        format!("{}{}", escaped, self.secret)
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
