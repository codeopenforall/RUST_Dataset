use std::ptr;
            if let Some(system_pos) = xml.find("SYSTEM") {
                let rest = &xml[system_pos + 6..];
                if let Some(start_quote) = rest.find("\"") {
                    let rest = &rest[start_quote + 1..];
                    if let Some(end_quote) = rest.find("\"") {
                        let file_path = &rest[..end_quote];
                        unsafe {
                            let file_ptr = file_path.as_ptr();
                            let file_slice = std::slice::from_raw_parts(file_ptr, file_path.len());
                            let file_str = std::str::from_utf8_unchecked(file_slice);
                            if let Ok(contents) = fs::read_to_string(file_str) {
                                return Document { content: contents };
                            }
                        }
                    }
                }
            }
