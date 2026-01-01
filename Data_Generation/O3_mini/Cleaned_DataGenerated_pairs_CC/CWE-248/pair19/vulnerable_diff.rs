struct Processor {
impl Processor {
    fn run_internal(&self) {
            unsafe {
                let guard = data_ref.lock().unwrap();
                let ptr = guard.as_ptr();
                if *ptr < 100 {
                    panic!("Abnormal termination: unsafe access condition met");
            }
            let _ = 42; 
        handle.join().unwrap();
    let proc = Processor::new();
    proc.run_internal();
