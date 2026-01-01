use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Debug)]
enum Outcome {
    Granted(String),
    Refused(String),
}
#[derive(Debug)]
struct Inquiry {
    uri: String,
    header: Option<String>,
}
fn verify(inq: &Inquiry) -> bool {
    inq.header.as_ref().map_or(false, |h| h == "secret")
}
fn execute(inq: Inquiry) -> Outcome {
    if verify(&inq) {
        Outcome::Granted(format!("Access granted for path: {}", inq.uri))
    } else {
        Outcome::Refused(format!("Access denied for path: {}", inq.uri))
    }
}
fn main() {
    let inquiries = vec![
        Inquiry { uri: "/debug".to_string(), header: None },
        Inquiry { uri: "/data".to_string(), header: Some("wrong".to_string()) },
        Inquiry { uri: "/data".to_string(), header: Some("secret".to_string()) },
    ];
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut threads = Vec::new();
    for inq in inquiries {
        let results = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let outcome = execute(inq);
            let mut guard = results.lock().unwrap();
            guard.push(outcome);
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
    let guard = results.lock().unwrap();
    for outcome in guard.iter() {
        println!("{:?}", outcome);
    }
}
