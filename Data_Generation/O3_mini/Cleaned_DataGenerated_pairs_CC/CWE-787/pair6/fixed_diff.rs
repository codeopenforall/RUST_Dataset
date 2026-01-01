        v.resize(size, 0);
        if index >= self.data.len() {
            return Err("Index out of bounds");
        if let Some(elem) = self.data.get_mut(index) {
            *elem = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    buf.assign(9, 999).expect("Assignment failed");
    println!("Assignment completed (fixed).");
