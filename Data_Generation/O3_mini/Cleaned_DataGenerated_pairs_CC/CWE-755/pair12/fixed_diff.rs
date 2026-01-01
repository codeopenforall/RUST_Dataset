    fn process(&self, input: i32) -> Result<i32, &'static str> {
            if input < 0 {
            }
            match p.process(inp) {
                Ok(result) => println!("Input {} => Output {}", inp, result),
                Err(e)    => println!("Input {} => Error: {}", inp, e),
            }
