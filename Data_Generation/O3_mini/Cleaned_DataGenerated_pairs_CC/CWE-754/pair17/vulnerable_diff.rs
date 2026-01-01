                if let ProcError::Timeout = e {
                    let default_box = Box::new(100);
                    let default_ptr = Box::into_raw(default_box);
                    let res = unsafe { *default_ptr };
                    Ok(res)
                } else {
                    Ok(0)
                }
        Err(e) => println!("Error: {:?}", e),
