use std::thread;
struct Resources {
    title: String,
    amount: u32,
}
impl Resources {
    fn new_complete() -> Self {
        Resources {
            title: String::from("configuration"),
            amount: 0,
        }
    }
}
fn run_app() -> u32 {
    let config = Resources::new_complete();
    let handle = thread::spawn(move || config.amount);
    handle.join().unwrap()
}
fn main() {
    let result = run_app();
    println!("Result: {}", result);
}
