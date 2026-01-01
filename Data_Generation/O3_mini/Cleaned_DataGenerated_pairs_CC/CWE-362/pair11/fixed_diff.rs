use std::sync::{Arc, Mutex};
    data: Arc<Mutex<i32>>,
        Controller { data: Arc::new(Mutex::new(initial)) }
    fn get(&self) -> i32 {
        *self.data.lock().unwrap()
    fn set(&self, val: i32) {
        let mut guard = self.data.lock().unwrap();
        *guard = val;
        let mut guard = self.data.lock().unwrap();
        if *guard == 0 {
            thread::sleep(Duration::from_millis(50));
            *guard = 1;
    let shared_ctl = Arc::new(ctl);
    let ctl_clone1 = Arc::clone(&shared_ctl);
    let ctl_clone2 = Arc::clone(&shared_ctl);
        ctl_clone1.process();
        let mut guard = ctl_clone2.data.lock().unwrap();
        if *guard == 0 {
            *guard = 2;
    let final_val = shared_ctl.get();
    println!("Final value: {}", final_val);
    assert!(final_val == 1, "Race condition fixed: final value is not 1");
