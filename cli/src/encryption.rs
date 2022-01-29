//use magic_crypt::MagicCryptTrait;
//use openssl::rsa::{Padding, Rsa};
use base64ct::{Base64Url, Encoding};
use sha2::{Digest, Sha256};
//use openssl::pkey::PKey;

pub fn decrypt_string(to_decrypt: &str) -> Result<String, Box<dyn std::error::Error>> {
    // entschlüsseln
    let password = keyring::Entry::new("envwoman", "envwoman")
        .get_password()
        .unwrap_or_else(|_| "Could not find password".to_string());
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let hash = hasher.finalize();
    let hash = Base64Url::encode_string(&hash);
    let fernet = fernet::Fernet::new(hash.as_str()).unwrap();
    return Ok(String::from_utf8(fernet.decrypt(to_decrypt.as_ref()).unwrap()).unwrap());
}

pub fn encrypt_string(to_encrypt: &str) -> Result<String, Box<dyn std::error::Error>> {
    // verschlüsseln
    let password = keyring::Entry::new("envwoman", "envwoman")
        .get_password()
        .unwrap_or_else(|_| "Could not find password".to_string());
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let hash = hasher.finalize();
    let hash = Base64Url::encode_string(&hash);
    let fernet = fernet::Fernet::new(hash.as_str()).unwrap();
    return Ok(String::from_utf8(fernet.decrypt(to_encrypt.as_ref()).unwrap()).unwrap());
}
/*
pub fn test() -> Result<(), Box<dyn std::error::Error>> {
    let mut hasher = Sha256::new();
    hasher.update("test".as_bytes());
    let hash = hasher.finalize();
    let hash = Base64Url::encode_string(&hash);
    println!("{}", hash);
    let fernet = fernet::Fernet::new(hash.as_str()).unwrap();
    let encrypted_data = fernet.encrypt("test".as_ref());
    println!("{}", encrypted_data);
    println!("{}", String::from_utf8(fernet.decrypt(encrypted_data.as_ref()).unwrap()).unwrap());
    Ok(())
}
*/
/*pub fn encrypt_string(to_encrypt: &str) -> Result<String, Box<dyn std::error::Error>> {
    // verschlüsseln
    let service = "envwoman";
    let username = "envwoman";
    let password = keyring::Entry::new(service, username).get_password()?;
    Ok("ok".into())
}


pub fn decrypt_string(to_decrypt: &str) -> Result<String, Box<dyn std::error::Error>> {
    // entschlüsseln
    let service = "envwoman";
    let username = "envwoman";
    let password = keyring::Entry::new(service, username).get_password()?;
    // let rsa = Rsa::generate(4096).unwrap();
    // let pkey = PKey::from_rsa(rsa).unwrap();
    Ok("res".into())
}


pub fn generate_key() -> Result<(), Box<dyn std::error::Error>> {
    // entschlüsseln
    // let password = keyring::Entry::new("envwoman", "envwoman").get_password()?;

    /*
        let rsa = Rsa::generate(4096).unwrap();
        let pkey = PKey::from_rsa(rsa.clone()).unwrap();
        println!("{:?}", String::from_utf8(pkey.rsa().unwrap().private_key_to_pem().unwrap()).unwrap());
        println!("\n{:?}", String::from_utf8(pkey.rsa().unwrap().public_key_to_pem().unwrap()).unwrap());
        let to_encrypt = "test";
        let mut buf = vec![0; rsa.size() as usize];
        let encrypted_size = rsa.private_encrypt(to_encrypt.as_bytes(), &mut buf, Padding::PKCS1).unwrap();
        let res_as_base64 = base64::encode_block(&buf);
        let old_buf = buf.clone();
        println!("{:?}", &res_as_base64);
        let mut buf = vec![0; rsa.size() as usize];
        rsa.private_decrypt(&old_buf, &mut buf, Padding::PKCS1).unwrap();
        println!("{:?}", String::from_utf8(buf).unwrap());
        */
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    println!("{:?}", private_key.to_pkcs1_der().unwrap().to_pkcs1_pem().unwrap());
    let data = b"hello world";
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let enc_data = public_key.encrypt(&mut rng, padding, &data[..]).expect("failed to encrypt");
    let base_64_res = base64::encode_block(&enc_data);
//    println!("{:?}", base_64_res);
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let dec_data = private_key.decrypt(padding, &base64::decode_block(&base_64_res).unwrap()).expect("failed to decrypt");
    println!("{:?}", String::from_utf8(dec_data).unwrap());

    Ok(())
}*/
