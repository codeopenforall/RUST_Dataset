struct Service {
impl Service {
        Service {
    fn execute(&self) -> io::Result<()> {
                unsafe {
                    let file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .mode(0o777) 
                        .open(&*path_clone)?;
                    writeln!(&file, "Sensitive Data")?;
                }
    let service = Service::new("tempfile.txt");
    service.execute()
