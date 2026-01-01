        if start.checked_add(count).unwrap_or(usize::MAX) > self.data.len() {
            panic!("Index out-of-bounds in compute");
        let slice = &self.data[start..start + count];
        slice.iter().sum()
        let _ = handle.join().unwrap_or_else(|err| {
            eprintln!("Thread panicked: {:?}", err);
            0
        });
