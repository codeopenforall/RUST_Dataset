        let target = self.base.join(filename);
            let bytes = target.to_str().unwrap().as_bytes();
        fs::read_to_string(target)
