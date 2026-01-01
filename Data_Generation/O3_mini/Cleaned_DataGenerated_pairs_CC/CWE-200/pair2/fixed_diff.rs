impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
         .field("name", &self.name)
         .field("api_key", &"********")
         .finish()
    }
}
    let log_output = format!("{:?}", conf);
