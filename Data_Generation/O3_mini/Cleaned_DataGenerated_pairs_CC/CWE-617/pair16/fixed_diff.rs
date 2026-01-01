use std::sync::{Arc, Mutex};
        debug_assert!(self.threshold < 100, "Threshold too high, possible DoS");
    if value >= 100 {
        panic!("Invalid threshold value");
    }
    let config = Arc::new(Mutex::new(System::new(value)));
    let config_clone = Arc::clone(&config);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let sys = config_clone.lock().unwrap();
        sys.process();
    {
        let sys = config.lock().unwrap();
        sys.process();
