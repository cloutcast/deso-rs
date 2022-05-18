const SEED_PHRASE: &str =  "lemon kidney crack item paper history lady column govern model tool auction";

#[test]
fn create_valid_seed_hex() {
    crate::seed_hex::from_mnemonic(SEED_PHRASE.to_string())
    .expect("could not create seed hex from mnemonic.");
}


#[test]
fn create_seed_hex_fails_on_empty_string() {
    match crate::seed_hex::from_mnemonic("".to_string()) {
        Ok(_) => panic!("should not pass"),
        Err(e) => assert_eq!(format!("{}", e), "could not create mnemonic object. is this a valid mnemonic?: invalid word in phrase".to_string())
    }
}


#[test]
fn create_seed_hex_fails_on_invalid_string() {
    match crate::seed_hex::from_mnemonic(format!("bad{}", SEED_PHRASE)) {
        Ok(_) => panic!("should not pass"),
        Err(e) => assert_eq!(format!("{}", e), "could not create mnemonic object. is this a valid mnemonic?: invalid word in phrase".to_string())
    }
}