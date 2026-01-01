    unsafe fn new(config: i32) -> Self {
                unsafe {
                    let locked = s.lock().unwrap();
                    let ptr = (&*locked) as *const i32;
                    *ptr
                }
            let _ = handle.join().unwrap();
fn process(input: &str) -> Engine {
    let config_val = input.parse::<i32>().unwrap_or_default(); 
    unsafe { Engine::new(config_val) }
    let engine = process(input);
    let engine = process("bad");
    engine.run();
