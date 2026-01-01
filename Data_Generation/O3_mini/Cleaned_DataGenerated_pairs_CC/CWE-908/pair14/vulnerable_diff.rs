struct Settings {
impl Settings {
    fn new() -> Self {
        Settings {
            threshold: 0,
            description: String::new(),
        }
fn load_resource(trigger: i32) -> Settings {
    let mut resource: MaybeUninit<Settings> = MaybeUninit::uninit();
        (*resource.as_mut_ptr()).threshold = trigger;
        resource.assume_init()
    let thread_result = handle.join().expect("Thread panicked");
        shared.threshold, thread_result
