use secp256k1::{SecretKey, Secp256k1, Message, PublicKey};
use sha2::{Sha256, Digest};

use crate::signing::*;

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

impl ToHexString for SeedHex {
    fn to_hex_string(&self) -> String {
        hex::encode(self.seed_bytes.to_owned())
    }
}

impl ToPublicKey for SeedHex {
    fn to_pub_string(&self) -> String {
        // self.secret_key.
        hex::encode(self.public_key.serialize())
    }

    fn to_pub_bytes(&self) -> Vec<u8> {
        self.public_key.serialize().to_vec()
    }
}


