use magic_crypt::MagicCryptTrait;

pub fn decrypt_string(to_decrypt: &str) -> Result<String, Box<dyn std::error::Error>> {
    // entschlüsseln
    let service = "envwoman";
    let username = "envwoman";
    let entry = keyring::Entry::new(service, username);
    let mc = new_magic_crypt!(entry.get_password()?, 256);
    let res = mc.decrypt_base64_to_string(to_decrypt)?;
    Ok(res)
}

pub fn encrypt_string(to_encrypt: &str) -> Result<String, Box<dyn std::error::Error>> {
    // verschlüsseln
    let service = "envwoman";
    let username = "envwoman";
    let entry = keyring::Entry::new(service, username);
    let mc = new_magic_crypt!(entry.get_password()?, 256);
    let base64_str = mc.encrypt_str_to_base64(to_encrypt);
    Ok(base64_str)
}
