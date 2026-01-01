const MAX_ITEMS: usize = 1_000_000;
         if count > MAX_ITEMS {
             return Err("Requested allocation exceeds limit");
         }
         let vec = vec![0xAA; count];
         Ok(vec)
