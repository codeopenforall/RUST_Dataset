    fn new_complete() -> Self {
        Resources {
            title: String::from("configuration"),
            amount: 0,
        }
    let config = Resources::new_complete();
    let handle = thread::spawn(move || config.amount);
