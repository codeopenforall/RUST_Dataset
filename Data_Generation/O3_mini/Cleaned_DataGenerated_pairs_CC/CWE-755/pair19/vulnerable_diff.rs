    fn load() -> Self {
        let value = env::var("CONFIG_VAL").unwrap_or_default(); 
        let secret = value.parse::<i32>().unwrap_or_default();   
        Config { secret }
    unsafe {
        let ptr = &config.secret as *const i32;
        let secret_val = *ptr;
        input / secret_val   
    }
fn execute() {
    let config = Config::load();
    let res = handle.join().unwrap();
    println!("Result: {}", res);
    execute();
