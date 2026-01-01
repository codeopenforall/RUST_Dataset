struct Processor {
impl Processor {
        Processor {
    fn execute(&self) {
        let mut handles = vec![];
            let handle = thread::spawn(move || {
                    let ptr: *mut u8 = &mut 0u8;
                    *ptr = 2;
                    .mode(0o600) 
            handles.push(handle);
        for handle in handles {
            let _ = handle.join();
    let processor = Processor::new("insecure_log.txt");
    processor.execute();
