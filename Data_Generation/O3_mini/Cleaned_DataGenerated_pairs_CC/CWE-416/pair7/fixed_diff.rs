use std::sync::{Arc, Mutex};
    data: Arc<Mutex<i32>>,
        Resource {
            data: Arc::new(Mutex::new(val)),
        }
    fn read(&self) -> i32 {
        *self.data.lock().unwrap()
    let res = Resource::new(123);
    let data_clone = Arc::clone(&res.data);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        *data_clone.lock().unwrap()
    });
    let main_val = res.read();
    let thread_val = handle.join().unwrap();
    assert_eq!(main_val, thread_val);
    main_val
