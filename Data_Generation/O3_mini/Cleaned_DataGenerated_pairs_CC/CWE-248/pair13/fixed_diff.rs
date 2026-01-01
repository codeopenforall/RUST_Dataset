use std::panic;
        match panic::catch_unwind(|| unsafe { (self.callback)(input) }) {
            Ok(result) => Some(result),
            Err(_) => None,
        }
        let result = proc.execute("trigger");
        match result {
            Some(val) => println!("{}", val),
            None => println!("Recovered from panic"),
        }
