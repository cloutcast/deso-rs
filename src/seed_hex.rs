use bip39::{Mnemonic, Language, Seed};
use hdwallet::{KeyChain, ExtendedPubKey, ChainPath};
use secp256k1::{SecretKey, Secp256k1, Message, PublicKey};
use sha2::{Sha256, Digest};

use crate::{DesoSigningError::{DesoSigningError,produce}, FromMnemonicError::{FromMnemonicError, produce as produce_n}};

pub struct SeedHex {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
    pub seed_bytes: Vec<u8>
}



impl SeedHex { 
    pub fn sign_transaction(&self, transaction_hex: &str) -> Result<String, DesoSigningError> {
        let seed_key = self.secret_key;
        let decoded_hash = match hex::decode(transaction_hex) {
            Ok(v) => v,
            Err(e) => return Err(produce(format!("could not decode transaction hex into string: {}", e)))
        };
        let hash1: &[u8] = &*Sha256::digest(decoded_hash);
        let hash2: &[u8] = &*Sha256::digest(hash1);
        let message = match Message::from_slice(hash2) {
            Ok(v) => v,
            Err(e) => return Err(produce(format!("could not create message to sign: {}", e)))
        };
        let secp = Secp256k1::new();    // create a secp256k1 object to work with.
        
        let sig = secp.sign_ecdsa(&message, &seed_key);

        let sig_der = sig.serialize_der();
        let sig_hex = hex::encode(sig_der);
        let mut final_txo_bytes: Vec<u8> = Vec::new();
        let mut sig_bytes = match hex::decode(&sig_hex) {
            Ok(v) => v,
            Err(e) => return Err(produce(format!("could not convert sig hex back to bytes: {}", e)))
        };
        let mut txo_bytes = match hex::decode(transaction_hex) {
            Ok(v) => v,
            Err(e) => return Err(produce(format!("could not make txhex vec: {}", e)))
        };

        let txo_sig_length_i: u64 = match sig_der.len().try_into() {
            Ok(v) => v, 
            Err(e) => return Err(produce(format!("could not determine sig_der length: {}", e)))
        };
        let txo_sig_hex = format!("{:X}", txo_sig_length_i);

        let mut txo_sig_vec = match hex::decode(txo_sig_hex) {
            Ok(v) => v,
            Err(e) => return Err(produce(format!("could not convert sig length hex into vec: {}", e)))
        };

        let new_txbyte_len = txo_bytes.len() - 1;
        txo_bytes.truncate(new_txbyte_len);

        final_txo_bytes.append(&mut txo_bytes);
        final_txo_bytes.append(&mut txo_sig_vec);
        final_txo_bytes.append(&mut sig_bytes);

        let final_sig_hex = hex::encode(&final_txo_bytes);

        Ok(final_sig_hex)
    }
}

pub fn from_mnemonic(mnemonic_phrase: String) -> Result<SeedHex, FromMnemonicError> {
    let mnemonic = match Mnemonic::from_phrase(&mnemonic_phrase, Language::English) {
        Ok(v) => v,
        Err(e) => return Err(produce_n(format!("could not create mnemonic object. is this a valid mnemonic?: {}", e)))
    };

    let seed = Seed::new(&mnemonic, "");

    let seed_bytes = seed.as_bytes();

    let master_key = match hdwallet::ExtendedPrivKey::with_seed(&seed_bytes) {
        Ok(v) => v,
        Err(e) => return Err(produce_n(format!("Could not create master key from seed: {:?}", e)))
    };

    let keychain = hdwallet::DefaultKeyChain::new(master_key);
    let chain_path: ChainPath =  "m/44'/0'/0'/0/0".into();
    let (child_key, _deriv) = match keychain.derive_private_key(chain_path){
        Ok(v) => v,
        Err(e) => return Err(produce_n(format!("could not derive child key from chain_path. err: {:?}", e)))
    };

    let prv: SecretKey = child_key.private_key;

    let pub_ext: ExtendedPubKey = ExtendedPubKey::from_private_key(&child_key);
    let pub_key = pub_ext.public_key;

    Ok(SeedHex {
        secret_key: prv,
        seed_bytes: seed_bytes.to_vec(),
        public_key: pub_key
    })
}
