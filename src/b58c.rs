use std::error::Error;



pub struct Base58CheckConfig {
    alphabet: String,
    base_map: [u8; 256],
    base: u64,
    leader: char,
    factor: u64,
    i_factor: u64
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
    let alphabet_length = alphabet.len();
    let int_alphabet_length: i64 = alphabet_length.try_into()?;
    
    for x in 0..int_alphabet_length {

        let char_at_usize: usize = x.try_into()?;
        // let char_at = char::from_u32(x_u32);
        let char_at = alphabet_vec[char_at_usize];
        let mut u8_bytes: [u8;2] = [0;2];
        char_at.encode_utf8(&mut u8_bytes);
        let char_byte = u8_bytes[0];
        let char_code_usize: usize = char_byte.try_into()?;
        let x_as_u8 : u8 = x.try_into()?;
        base_map[char_code_usize] = x_as_u8;
        // let mut char_code = char_at.to_digit(radix)
    }

    let base = alphabet.len();
    let base_u8: u8 = base.try_into()?;

    let leader = alphabet_vec[0];
    let log_256 = (256 as f32).log(std::f32::consts::E);

    let float_base: f32 = base_u8.try_into()?;
    let base_log = float_base.log(std::f32::consts::E);
    let factor_f = base_log / log_256;

    let factor = factor_f.round() as u64;

    let i_factor_f = log_256 / base_log;

    let i_factor = i_factor_f.round() as u64;


    let b = Base58CheckConfig {
        alphabet: alphabet,
        base_map: base_map,
        base: base as u64,
        leader: leader,
        factor: factor,
        i_factor: i_factor
    };
    Ok(b)
}

pub fn encode_b58c_plain(source: Vec<u8>) -> Result<String, Box<dyn Error>> {

    if source.len() <= 0 {
        return Ok("".to_string());
    }

    let mut zeroes: u64 = 0;
    let mut length: u64 = 0;
    let mut pbegin: u64 = 0;

    let pend = source.len() as u64;

    while pbegin != pend && source[pbegin as usize] == 0 {
        pbegin += 1;
        zeroes += 1;
    }

    let fake_res = "lol".to_string();
    Ok(fake_res)
}

