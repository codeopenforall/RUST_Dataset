use std::io::Write;
    unsafe fn update(&self, val: u32) {
        let _guard = lock.lock().unwrap();
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("data.log") {
            let _ = write!(file, "{}", val);
        }
    fn run(&self, iterations: u32) {
            let handle = thread::spawn(move || {
                    let mut guard = lock.lock().unwrap();
                if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("data.log") {
                    let _ = write!(file, "{}", i);
                }
            let _ = th.join();
fn main() {
        worker.update(100);
    worker.run(10);
