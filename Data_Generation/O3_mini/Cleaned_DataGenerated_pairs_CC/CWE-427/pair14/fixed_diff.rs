    fn execute(&self) -> Result<String, String> {
        let secure_path = "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin";
        env::set_var("PATH", secure_path);
            match guard.execute() {
                Ok(result) => println!("Result: {}", result),
                Err(err) => eprintln!("Error: {}", err),
