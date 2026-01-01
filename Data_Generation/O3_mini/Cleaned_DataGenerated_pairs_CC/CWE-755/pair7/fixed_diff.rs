    fn new(config: i32) -> Self {
                let locked = s.lock().unwrap();
                *locked
            let res = handle.join();
            if res.is_err() {
                panic!("Thread encountered a problem");
            }
fn process(input: &str) -> Result<Engine, String> {
    let config_val = input
        .parse::<i32>()
        .map_err(|e| format!("Configuration parse error: {}", e))?;
    Ok(Engine::new(config_val))
    let engine = process(input)?;
    match process("bad") {
        Ok(engine) => engine.run(),
        Err(e) => {
            eprintln!("Failed to initialize engine: {}", e);
            std::process::exit(1);
        }
    }
