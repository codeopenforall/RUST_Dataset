fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}
        let combined = format!("{}{}", self.api_key, self.db_password);
            let combo_ptr = combined.as_ptr();
            let _ = std::slice::from_raw_parts(combo_ptr, combined.len());
        let encrypted = xor_encrypt(combined.as_bytes(), 0xAA);
        file.write_all(&encrypted)?;
