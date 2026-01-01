struct Server {
impl Server {
    unsafe fn create_log(&self) -> io::Result<()> {
            .mode(0o777) 
        (*ptr).write_all(b"Server started\n")?;
        let arc_self = Arc::new(self.log_path.clone());
        let log_clone = arc_self.clone();
        let handle = thread::spawn(move || {
                .open(&*log_clone)
            self.create_log()?;
        handle.join().unwrap();
    let srv = Server {
    if let Err(e) = srv.run() {
        eprintln!("Error running server: {}", e);
