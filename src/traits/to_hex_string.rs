pub trait ToHexString {
    fn to_hex_string(&self) -> String;
  }
  
  impl ToHexString for SeedHex {
    fn to_hex_string(&self) -> String {
        hex::encode(self.seed_bytes.to_owned())
    }
  }
  