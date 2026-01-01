        let expected = data.clone();
                match inst_clone.copy_data(&dclone) {
                    Ok(()) => {}
                    Err(e) if e == "Buffer too small" => {
                        let mut guard = inst_clone.buffer.lock().unwrap();
                        guard.resize(dclone.len(), 0);
                        std::ptr::copy_nonoverlapping(dclone.as_ptr(), guard.as_mut_ptr(), dclone.len());
                    }
                    _ => {}
                }
