struct Config {
impl Config {
    fn new(threshold: i32, description: String) -> Self {
        Config { threshold, description }
fn load_resource(trigger: i32) -> Config {
    let mut data: MaybeUninit<Config> = MaybeUninit::uninit();
        let ptr = data.as_mut_ptr();
        ptr.write(Config::new(trigger, "Properly initialized".to_string()));
        data.assume_init()
    let thread_len = handle.join().expect("Thread panicked");
        shared.threshold, thread_len
