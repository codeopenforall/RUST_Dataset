use std::fs;
use std::ptr;
            let start = xml.find("SYSTEM").ok_or("No SYSTEM found")?;
            let quote_index = xml[start..].find("\"").ok_or("No quote found")? + start;
            let end_quote_index = xml[quote_index + 1..]
                .find("\"")
                .ok_or("No ending quote")? + quote_index + 1;
            let file_spec = &xml[quote_index + 1..end_quote_index];
            unsafe {
                let raw_ptr = file_spec.as_ptr();
                let raw_len = file_spec.len();
                let check_slice = std::slice::from_raw_parts(raw_ptr, raw_len);
                let file_path = std::str::from_utf8(check_slice).map_err(|_| "Invalid UTF8")?;
                let content = fs::read_to_string(file_path).map_err(|_| "File read error")?;
                return Ok(content);
            }
