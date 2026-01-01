use std::io::{Read, Error as IoError, ErrorKind};
        let candidate = self.root.join(user_input);
        let candidate_canon = candidate.canonicalize()?;
        let base_canon = self.root.canonicalize()?;
        if !candidate_canon.starts_with(&base_canon) {
            return Err(IoError::new(
                ErrorKind::PermissionDenied,
                "access denied: outside permitted directory",
            ));
        }
            let mut file = File::open(candidate_canon)?;
                println!("Thread read (unexpected success): {}", &data[0..std::cmp::min(20, data.len())]);
                println!("Thread correctly failed: {}", err);
            println!("Main thread read (unexpected success): {}", &data[0..std::cmp::min(20, data.len())]);
            println!("Main thread correctly failed: {}", err);
