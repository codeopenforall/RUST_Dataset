use std::sync::{Arc, Mutex};
        if self.counter < 10 {
            self.counter += 1;
            thread::sleep(Duration::from_millis(1));
            if self.counter == 10 {
                self.finished = true;
    let worker = Arc::new(Mutex::new(Worker::new()));
        let w = Arc::clone(&worker);
            let mut worker = w.lock().unwrap();
            worker.process();
    let final_state = worker.lock().unwrap();
    (final_state.counter, final_state.finished)
