                    let mut guard = self.state.lock().unwrap();
                    *guard = Phase::Running;
                    return true;
