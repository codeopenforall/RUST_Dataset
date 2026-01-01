        let mut buffer = vec![0u32; 50];
                unsafe {
                    let ptr = buffer.as_mut_ptr();
                    *ptr.add(i) = 42;
                }
