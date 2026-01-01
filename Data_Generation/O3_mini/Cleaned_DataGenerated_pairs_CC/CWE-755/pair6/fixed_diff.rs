fn read_config(input: &str) -> Result<*mut ConfigStruct, &'static str> {
    let num = input.trim().parse::<u32>().map_err(|_| "Configuration parse failure")?;
    Ok(Box::into_raw(config))
fn retrieve_config() -> Result<u32, &'static str> {
            return Err("Global configuration not set");
        Ok((*GLOBAL_SETTING).value)
    let config_val = retrieve_config()?;
        return Err("Invalid configuration: value cannot be zero");
fn execute() -> Result<(), &'static str> {
        GLOBAL_SETTING = read_config("not_a_number")?;
        compute(100)
    let result = handle.join().map_err(|_| "Thread failed")??;
    Ok(())
    if let Err(e) = execute() {
        eprintln!("Error: {}", e);
    }
