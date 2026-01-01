    fn load() -> Result<Self, &'static str> {
        let value = env::var("CONFIG_VAL").map_err(|_| "missing CONFIG_VAL")?;
        let secret = value.parse::<i32>().map_err(|_| "CONFIG_VAL not an integer")?;
        if secret == 0 {
            return Err("CONFIG_VAL cannot be zero");
        }
        Ok(Config { secret })
    input / config.secret
fn execute() -> Result<(), &'static str> {
    let config = Config::load()?;
    let _res = handle.join().map_err(|_| "Thread panicked")?;
    println!("Execution completed without panic.");
    Ok(())
    if let Err(e) = execute() {
        eprintln!("Error encountered: {}", e);
        std::process::exit(1);
    }
