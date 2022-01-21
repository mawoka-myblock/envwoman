use std::str;

use aes::Aes256;
use base64ct::{Base64, Encoding};
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use confy;
use hmac::Hmac;
use magic_crypt::MagicCryptTrait;
use pbkdf2::pbkdf2;
use rand::RngCore;
use sha2::Sha256;

use crate::config;

/*pub fn encrypt_string(to_encrypt: &String) -> Result<String, Box<dyn std::error::Error>> { // verschlüsseln
    let mut key = [0u8; 32];
    let cfg: config::Config = confy::load("envwoman")?;
    let service = "envwoman";
    let username = &cfg.api_key;
    let entry = keyring::Entry::new(&service, &username);
    pbkdf2::<Hmac<Sha256>>(&entry.get_password()?.as_ref(), (&cfg.salt).as_ref(), 10, &mut key);
    let mut iv = [0u8; 16];
    // rand::thread_rng().fill_bytes(&mut iv);
    let cipher = Cbc::<Aes256, Pkcs7>::new_from_slices(&key, &iv).unwrap();
    let enc = cipher.encrypt_vec(&to_encrypt.as_bytes());
    let mut msg = Vec::with_capacity(iv.len() + enc.len());
    msg.extend_from_slice(&iv);
    msg.extend_from_slice(&enc);
    return Ok(Base64::encode_string(&msg));
}

pub fn decrypt_string(to_decrypt: &String) -> Result<String, Box<dyn std::error::Error>> { // entschlüsseln
    let mut key = [0u8; 32];
    let cfg: config::Config = confy::load("envwoman")?;
    let service = "envwoman";
    let username = &cfg.api_key;
    let entry = keyring::Entry::new(&service, &username);
    // pbkdf2::<Hmac<Sha256>>(&entry.get_password()?.as_ref(), (&cfg.salt).as_ref(), 10, &mut key);
    let mut iv = [0u8; 16];
    // rand::thread_rng().fill_bytes(&mut iv);
    println!("{}", to_decrypt);
    let cipher = Cbc::<Aes256, Pkcs7>::new_from_slices(&key, &iv).unwrap();
    let enc = match cipher.decrypt_vec(&to_decrypt.as_bytes()) {
        Ok(x) => x,
        Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Decryption failed"))),
    };
    return Ok(String::from_utf8(enc).unwrap());
}*/

pub fn decrypt_string(to_decrypt: &String) -> Result<String, Box<dyn std::error::Error>> {
    // entschlüsseln
    let cfg: config::Config = confy::load("envwoman")?;
    let service = "envwoman";
    let username = &cfg.api_key;
    let entry = keyring::Entry::new(&service, &username);
    let mc = new_magic_crypt!(entry.get_password()?, 256);
    let res = mc.decrypt_base64_to_string(to_decrypt).unwrap();
    Ok(res)
}

pub fn encrypt_string(to_encrypt: &String) -> Result<String, Box<dyn std::error::Error>> {
    // verschlüsseln
    let cfg: config::Config = confy::load("envwoman")?;
    let service = "envwoman";
    let username = &cfg.api_key;
    let entry = keyring::Entry::new(&service, &username);
    let mc = new_magic_crypt!(entry.get_password()?, 256);
    let base64_str = mc.encrypt_str_to_base64(to_encrypt);
    Ok(base64_str)
}
