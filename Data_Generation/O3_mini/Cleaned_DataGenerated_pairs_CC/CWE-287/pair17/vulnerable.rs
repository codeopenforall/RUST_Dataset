use std::env;
use std::sync::Arc;
use std::thread;
pub fn verify(token: &str) -> bool {
    unsafe {
        let expected = "secret";
        if token.len() < 3 {
            return false;
        }
        let token_ptr = token.as_ptr();
        let expected_ptr = expected.as_ptr();
        for i in 0..3 {
            if *token_ptr.add(i) != *expected_ptr.add(i) {
                return false;
            }
        }
        true
    }
}
fn process(auth: Arc<String>) {
    let result = verify(&auth);
    if result {
        println!("Access Granted");
    } else {
        println!("Access Denied");
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let token_input = if args.len() > 1 {
        args[1].clone()
    } else {
        String::new()
    };
    let token_arc = Arc::new(token_input);
    let mut handles = vec![];
    for _ in 0..4 {
        let token_clone = Arc::clone(&token_arc);
        handles.push(thread::spawn(move || process(token_clone)));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
