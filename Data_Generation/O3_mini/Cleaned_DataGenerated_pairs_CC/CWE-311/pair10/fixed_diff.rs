    fn xor_transform(input: &str, key: u8) -> String {
        let transformed: Vec<u8> = input.bytes().map(|b| b ^ key).collect();
        transformed.iter().map(|b| format!("{:02x}", b)).collect()
    }
        let key: u8 = 0xAA; 
        Self::xor_transform(&self.data, key)
