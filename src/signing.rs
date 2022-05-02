//! DeSo Signing Utilities


extern crate secp256k1;

use bip39::{Mnemonic, Language, Seed};
use hdwallet::{KeyChain, ExtendedPubKey};
use secp256k1::SecretKey;
use crate::seed_hex::SeedHex;

enum DesoSigningError {

}

pub trait ToHexString {
  fn to_hex_string(&self) -> String;
}

pub trait ToPublicKey {
  fn to_pub_string(&self) -> String;
  fn to_pub_bytes(&self) -> Vec<u8>;
}


pub trait MnemonicStringToSeedHex {
  fn to_seed_hex(&self) -> SeedHex;
}

impl MnemonicStringToSeedHex for str {
  fn to_seed_hex(&self) -> SeedHex {
    let mnemonic_phrase = format!("{}", self);
    mnemonic_to_seed_hex(mnemonic_phrase).expect("Could not convert mnemonic str to seed hex")
  }
}

impl MnemonicStringToSeedHex for String {
  fn to_seed_hex(&self) -> SeedHex {
    let mnemonic_phrase = format!("{}", self);
    mnemonic_to_seed_hex(mnemonic_phrase).expect("Could not convert mnemonic String to seed hex")
  }
}

fn mnemonic_to_seed_hex(mnemonic_phrase: String) -> Result<SeedHex, std::io::Error> {
  let mnemonic = Mnemonic::from_phrase(&mnemonic_phrase, Language::English)
      .expect("could not create mnemonic object. is this a valid mnemonic?");

  let seed = Seed::new(&mnemonic, "");

  let seed_bytes = seed.as_bytes();
  
  let master_key = hdwallet::ExtendedPrivKey::with_seed(&seed_bytes).expect("Could not create master key from seed");
  let keychain = hdwallet::DefaultKeyChain::new(master_key);
  let (child_key, _deriv) = keychain.derive_private_key("m/44'/0'/0'/0/0".into()).expect("could not derive child key");

  // hdwallet 0.3.0 and secp256k1 0.19.0 are interoperable at these specific versions. (hdwallet secretkey ==> secp256k1 secretkey)
  let prv: SecretKey = child_key.private_key;

  let pub_ext: ExtendedPubKey = ExtendedPubKey::from_private_key(&child_key);
  let pub_key = pub_ext.public_key;

  Ok(SeedHex {
    secret_key: prv,
    seed_bytes: seed_bytes.to_vec(),
    public_key: pub_key
  })
}


/*
pub trait SignTransaction {
  fn sign_transaction(&self, transaction_hex: &str) -> String;
}
*/

