use std::error::Error;
use sha2::{Digest, Sha256};


pub struct Base58CheckConfig {
    //alphabet: String,
    alphabet_vec: Vec<char>,
    //base_map: [u8; 256],
    base: u64,
    leader: char,
    //factor: u64,
    i_factor: usize
}

fn generate_base58check_config() -> Result<Base58CheckConfig, Box<dyn std::error::Error>> {

    let alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string();
    let alphabet_vec: Vec<char> = alphabet.chars().collect();

    let mut base_map: [u8; 256] = [0; 256];
    let int_bm_length = base_map.len() as i64;
    for x in 0..int_bm_length {
        base_map[x as usize] = 255;
    }
    let alphabet_length = alphabet.len();
    let int_alphabet_length = alphabet_length as u64;
    
    for x in 0..int_alphabet_length {

        let char_at_usize: usize = x as usize;        
        let char_at = alphabet_vec[char_at_usize];
        let mut u8_bytes: [u8;2] = [0;2];
        char_at.encode_utf8(&mut u8_bytes);
        let char_byte = u8_bytes[0];
        let char_code_usize = char_byte as usize;
        let x_as_u8 = x as u8;
        base_map[char_code_usize] = x_as_u8;
    }

    let base = alphabet.len();
    let base_u8 = base as u8;
    let log_256 = (256 as f32).log(std::f32::consts::E);

    let float_base: f32 = base_u8 as f32;
    let base_log = float_base.log(std::f32::consts::E);
    //let factor_f = base_log / log_256;
    //let factor = factor_f.round() as u64;

    let i_factor_f = log_256 / base_log;

    let leader =  alphabet_vec[0]; //ownership bullshit

    println!("{} {} {} {}", i_factor_f, leader, base, log_256);

    
    let b = Base58CheckConfig {
        //alphabet: alphabet,
        alphabet_vec: alphabet_vec,
        //base_map: base_map,
        base: base as u64,
        leader: leader,
        //factor: factor,
        i_factor: i_factor_f.round() as usize
    };
    Ok(b)
}

pub fn encode_b58c_plain(source: Vec<u8>) -> Result<String, Box<dyn Error>> {

    let b_config = generate_base58check_config()?;

    if source.len() <= 0 { return Ok("".to_string()); }

    let mut zeroes: usize = 0;
    let mut length: usize = 0;
    let mut pbegin: usize = 0;

    let pend = source.len();

    while pbegin != source.len() && source[pbegin] == 0 {
        pbegin += 1;
        zeroes += 1;
    }

    let size = ((pend - pbegin) * b_config.i_factor + 1) >> 0;

    let mut b58: Vec<u8> = vec![0; size as usize]; // https://stackoverflow.com/questions/29530011/creating-a-vector-of-zeros-for-a-specific-size
    
    while pbegin != pend {
        let mut carry = source[pbegin] as u64;
        
        let mut it1 = size - 1;
        let mut i = 0 as usize;
        while (carry != 0 || i < length) && it1 != 1 {
            carry += (256 * b58[it1] as u64) >> 0;
            b58[it1] = (carry % b_config.base >> 0) as u8;
            carry = (carry / b_config.base) >> 0;

            // end for
            it1 -= 1;
            i += 1;
        }

        if carry != 0 {
            panic!("carry is non-zero: {} {} ", carry, b_config.i_factor);
        }
        length = i;
        pbegin += 1;
    }

    // skip leading zeroes in base58 result. (line 64)
    let mut it2 = size - length;
    while it2 != size && b58[it2] == 0 {
        it2 += 1;
    }
    let mut str_leader = (format!("{}", b_config.leader)).as_str().repeat(zeroes);

    while it2 < size {
        it2 += 1;

        let str_slice = format!("{}", b_config.alphabet_vec[b58[it2] as usize]); 
        str_leader = format!("{}{}", str_leader, str_slice);
    }    

    Ok(str_leader)
}

fn double_sha256(payload: &[u8]) -> Vec<u8> {
    let hasher = Sha256::new().chain_update(&payload);
    let output: Vec<_> = hasher.finalize().into_iter().collect();

    let hasher = Sha256::new().chain_update(&output);
    hasher.finalize().into_iter().collect()
}

pub fn encode(payload: Vec<u8>) -> Result<String, Box<dyn Error>> {
    let payload_u8a: &[u8] = &*payload;
    let checksum = double_sha256(payload_u8a);

    let mut the_buffer: Vec<u8> = vec![0; payload.len() + 4];
    
    let xx = payload_u8a.len();
 
    // the_buffer.copy_from_slice(payload_u8a);
    
    let ii = 0;
    for i in ii..xx {
        the_buffer[i] = payload_u8a[i];
    }

    the_buffer[ xx + 0] = checksum[0];
    the_buffer[ xx + 1] = checksum[1];
    the_buffer[ xx + 2] = checksum[2];
    the_buffer[ xx + 3] = checksum[3];

    encode_b58c_plain(the_buffer)
}