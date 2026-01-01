struct Engine {
impl Engine {
        Engine { base }
    fn process(&self, input: &str) -> i32 {
        let divisor: i32 = input.parse().unwrap_or_default();  
            value / divisor
    let engine = Engine::new(100);
    Ok(engine.process(input))
    let engine = Arc::new(Engine::new(100));
        let engine = Arc::clone(&engine);
            let res = engine.process(&arg_clone);
            println!("Computed result: {}", res);
