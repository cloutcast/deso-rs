use bip39::{Mnemonic, Language, Seed};
use hdwallet::{KeyChain, ExtendedPubKey};
use secp256k1::{SecretKey, Secp256k1, Message, PublicKey};
use sha2::{Sha256, Digest};

pub enum DesoSigningError {

}
  
pub struct SeedHex {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
    pub seed_bytes: Vec<u8>
}

impl SeedHex { 
    pub fn sign_transaction(&self, transaction_hex: &str) -> Result<String, std::io::Error> {
        let seed_key = self.secret_key;
        let hash1: &[u8] = &*Sha256::digest(hex::decode(transaction_hex).expect("hex could not be converted"));
        let hash2: &[u8] = &*Sha256::digest(hash1);
        let message = Message::from_slice(hash2).expect("could not create message to sign");
        let secp = Secp256k1::new();    // create a secp256k1 object to work with.
        let sig = secp.sign(&message, &seed_key);
        let sig_der = sig.serialize_der();
        let sig_hex = hex::encode(sig_der);
        let mut final_txo_bytes: Vec<u8> = Vec::new();
        let mut sig_bytes = hex::decode(&sig_hex).expect("could not convert sig hex back to bytes..");
        let mut txo_bytes = hex::decode(transaction_hex).expect("could not make txhex vec");

        let txo_sig_length_i: u64 = sig_der.len().try_into().unwrap();
        let txo_sig_hex = format!("{:X}", txo_sig_length_i);
        let mut txo_sig_vec = hex::decode(txo_sig_hex).expect("could not convert sig length hex into vec");

        let new_txbyte_len = txo_bytes.len() - 1;
        txo_bytes.truncate(new_txbyte_len);


        final_txo_bytes.append(&mut txo_bytes);
        final_txo_bytes.append(&mut txo_sig_vec);
        final_txo_bytes.append(&mut sig_bytes);


        let final_sig_hex = hex::encode(&final_txo_bytes);

        Ok(final_sig_hex)
    }
}

pub fn from_mnemonic(mnemonic_phrase: String) -> Result<SeedHex, std::io::Error> {
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
