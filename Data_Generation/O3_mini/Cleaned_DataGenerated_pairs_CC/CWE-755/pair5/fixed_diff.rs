    pub fn new() -> Result<Self, &'static str> {
        let cfg = load_config()?;
        Ok(Processor { config: cfg })
        self.config
            .factor
            .checked_mul(val)
            .unwrap_or_else(|| 0)
    let processor = Processor::new()?;
    Ok(result)
