            let ptr = self.inner.as_mut_ptr().offset(start);
                *ptr.offset(i as isize) = val;
    match obj.modify(0, 12, 42) {
        Ok(_) => println!("Modification completed."),
