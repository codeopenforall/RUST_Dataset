use std::thread;
            },
            1 | _ => {
                let base_ptr = &self.flag as *const Arc<Mutex<u32>>;
                unsafe {
                    let bad_ptr = base_ptr.offset(1);
                    if bad_ptr.is_null() {
                        return Err("null pointer encountered");
                    }
                    let misplaced_ref: &Arc<Mutex<u32>> = &*bad_ptr;
                    let cnt = misplaced_ref.lock().map_err(|_| "mutex poisoned")?;
                    Ok(*cnt)
                }
            },
