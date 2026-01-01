                RecvTimeoutError::Timeout => {
                    println!("Ignoring timeout condition");
                    Ok(0)
                },
        Ok(val) => println!("Operation completed with: {}", val),
        Err(e) => eprintln!("Error encountered: {}", e),
