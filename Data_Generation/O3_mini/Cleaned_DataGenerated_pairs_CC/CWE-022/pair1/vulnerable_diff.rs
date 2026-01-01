use std::io::{Read, Error as IoError};
        let joined = self.root.join(user_input);
            let mut file = File::open(joined)?;
                println!("Thread completed read: {}", &data[0..std::cmp::min(20, data.len())]);
                eprintln!("Thread got error: {}", err);
            println!("Main thread read: {}", &data[0..std::cmp::min(20, data.len())]);
            eprintln!("Main thread error: {}", err);
