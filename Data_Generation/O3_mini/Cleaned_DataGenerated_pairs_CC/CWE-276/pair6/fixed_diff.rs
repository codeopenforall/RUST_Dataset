struct Module {
impl Module {
        Module {
    fn run(&self) -> io::Result<()> {
                let file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .mode(0o600)
                    .open(&*path_clone)?;
                writeln!(&file, "Sensitive Data")?;
    let module = Module::new("tempfile.txt");
    module.run()
