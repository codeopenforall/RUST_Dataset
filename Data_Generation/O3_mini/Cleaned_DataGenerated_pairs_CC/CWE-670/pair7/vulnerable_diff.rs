                    unsafe {
                        let mut guard = self.state.lock().unwrap();
                        let ptr: *mut Phase = &mut *guard;
                        *ptr = Phase::Completed;
                    }
                    return false;
