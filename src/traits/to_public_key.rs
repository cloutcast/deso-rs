pub trait ToPublicKey {
    fn to_pub_string(&self) -> String;
    fn to_pub_bytes(&self) -> Vec<u8>;
}
  
impl ToPublicKey for SeedHex {
    fn to_pub_string(&self) -> String {    
        hex::encode(self.public_key.serialize())
    }

    fn to_pub_bytes(&self) -> Vec<u8> {
        self.public_key.serialize().to_vec()
    }
}