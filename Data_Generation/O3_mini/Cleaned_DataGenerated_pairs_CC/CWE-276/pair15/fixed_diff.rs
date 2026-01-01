struct Handler {
impl Handler {
    unsafe fn initialize_log(&self) -> io::Result<()> {
            .mode(0o600) 
        (*ptr).write_all(b"Handler started\n")?;
        let arc_path = Arc::new(self.log_path.clone());
        let path_clone = arc_path.clone();
        let thread_handle = thread::spawn(move || {
                .open(&*path_clone)
            self.initialize_log()?;
        thread_handle.join().unwrap();
    let h = Handler {
    if let Err(e) = h.run() {
        eprintln!("Error during processing: {}", e);
