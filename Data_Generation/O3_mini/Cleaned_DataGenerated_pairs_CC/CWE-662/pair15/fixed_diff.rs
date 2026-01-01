                let mut guard = s.lock().unwrap();
                guard.value = guard.value.wrapping_add(1);
