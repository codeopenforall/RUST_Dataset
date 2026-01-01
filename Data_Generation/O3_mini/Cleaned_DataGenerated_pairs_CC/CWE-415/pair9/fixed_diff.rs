    fn from_value(val: i32) -> Self {
        let boxed = Box::new(val);
        Item { data: Box::into_raw(boxed) }
    }
        let val = self.value();
        Item::from_value(val)
            drop(Box::from_raw(self.data));
