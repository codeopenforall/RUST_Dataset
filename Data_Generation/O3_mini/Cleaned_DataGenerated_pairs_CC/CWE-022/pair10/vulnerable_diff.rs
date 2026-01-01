use std::io::{Read, Result};
        let mut path = self.root.clone();
        path.push(req);
            let mut file = File::open(&path)?;
