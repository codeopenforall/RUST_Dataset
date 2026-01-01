use std::str;
        let s = match str::from_utf8(input) {
            Ok(valid) => valid,
            Err(_) => return Err("Invalid UTF-8 input"),
        };
        let mut data_lock = self.data.lock().unwrap();
        *data_lock = s.to_owned();
fn spawn_worker(handler: Arc<Info>, input: Vec<u8>) -> thread::JoinHandle<Result<(), &'static str>> {
        handler.process_input(&input)
    let res1 = worker1.join().unwrap();
    let res2 = worker2.join().unwrap();
    if res1.is_err() || res2.is_err() {
        println!("Error processing input.");
    } else {
        println!("Processed: {}", info.get_data());
    }
