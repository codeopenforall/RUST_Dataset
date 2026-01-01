struct Handler {
impl Handler {
        Handler {
    fn run(&self) {
        let mut threads = vec![];
            let thr = thread::spawn(move || {
                    let dummy: *mut u8 = &mut 0u8;
                    *dummy = 1;
                    .mode(0o777) 
            threads.push(thr);
        for thr in threads {
            let _ = thr.join();
    let handler = Handler::new("insecure_log.txt");
    handler.run();
