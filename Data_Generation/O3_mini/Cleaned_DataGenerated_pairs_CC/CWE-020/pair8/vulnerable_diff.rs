        unsafe {
            let s = std::str::from_utf8_unchecked(input);
            let mut data_lock = self.data.lock().unwrap();
            *data_lock = s.to_owned();
        }
fn spawn_worker(handler: Arc<Info>, input: Vec<u8>) -> thread::JoinHandle<()> {
        let _ = handler.process_input(&input);
    worker1.join().unwrap();
    worker2.join().unwrap();
    println!("Processed: {}", info.get_data());
