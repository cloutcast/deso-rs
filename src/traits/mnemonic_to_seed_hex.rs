use crate::seed_hex::from_mnemonic;

pub trait MnemonicToSeedHex {
    fn to_seed_hex(&self) -> SeedHex;
}

impl MnemonicToSeedHex for str {
    fn to_seed_hex(&self) -> SeedHex {
        let mnemonic_phrase = format!("{}", self);
        SeedHex::from_mnemonic(mnemonic_phrase).expect("Could not convert mnemonic str to seed hex")
    }
}

impl MnemonicToSeedHex for String {
    fn to_seed_hex(&self) -> SeedHex {
        let mnemonic_phrase = format!("{}", self);
        SeedHex::from_mnemonic(mnemonic_phrase).expect("Could not convert mnemonic String to seed hex")
    }
}
  
  