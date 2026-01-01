                        if e.kind() == ErrorKind::WouldBlock {
                            *d = 1000;
                            Ok(1000)
                            *d = 1000;
                            Ok(1000)
