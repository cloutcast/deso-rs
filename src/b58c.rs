use core::slice::SlicePattern;



pub struct Base58CheckConfig {
    alphabet: String,
    base_map: [u8; 256],
    base: u64,
    leader: char,
    factor: i128,
    i_factor: i128
}

pub fn generate_base58check_config() -> Result<Base58CheckConfig, Box<dyn std::error::Error>> {

    let alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string();
    let alphabet_vec: Vec<char> = alphabet.chars().collect();

    let mut base_map: [u8; 256] = [0; 256];
    let int_bm_length: i64 = base_map.len().try_into()?;
    for x in 0..int_bm_length {
        let usize_x: usize = x.try_into()?;
        base_map[usize_x] = 255;
    }

    let int_alphabet_length: i64 = alphabet.len().try_into()?;
    
    for x in 0..int_alphabet_length {
        let usize_x: usize = x.try_into()?;

        let char_at = alphabet_vec[usize_x].to_string().as_str().as_bytes();

    }




    let b = Base58CheckConfig {
        alphabet: alphabet,
        base_map: base_map,
        base: 256,
        leader: 'a',
        factor: 0,
        i_factor: 0
    };
    Ok(b)
}