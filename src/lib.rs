use wasm_bindgen::prelude::*;
use openssl::aes::{AesKey, AesDecrypt};
use openssl::rsa::{Rsa, RsaPadding};
use openssl::base64;
use openssl::rand::rand_bytes;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[wasm_bindgen]
pub fn generate_aes_key() -> String {
    let mut key = vec![0; 16];
    rand_bytes(&mut key).expect("Failed to generate key");
    let filename = format!("aes-{}.key", get_timestamp());
    fs::write(&filename, key).expect("Unable to write key to file");
    filename
}

#[wasm_bindgen]
pub fn encrypt_aes(data: &str, key_file: &str) -> String {
    let key = fs::read(key_file).expect("Unable to read key file");
    let cipher = AesKey::new_encrypt(&key).expect("Unable to create AES cipher");
    let data_bytes = data.as_bytes();
    let mut buffer = vec![0; data_bytes.len()];
    cipher.encrypt(data_bytes, &mut buffer).expect("Encryption failed");
    base64::encode(&buffer)
}

#[wasm_bindgen]
pub fn decrypt_aes(data: &str, key_file: &str) -> String {
    let key = fs::read(key_file).expect("Unable to read key file");
    let cipher = AesKey::new_decrypt(&key).expect("Unable to create AES cipher");
    let data_bytes = base64::decode(data).expect("Invalid base64 data");
    let mut buffer = vec![0; data_bytes.len()];
    cipher.decrypt(&data_bytes, &mut buffer).expect("Decryption failed");
    String::from_utf8(buffer).expect("Invalid UTF-8 data")
}

#[wasm_bindgen]
pub fn generate_rsa_key() -> String {
    let rsa = Rsa::generate(2048).expect("Failed to generate RSA key");
    let priv_key_pem = rsa.private_key_to_pem().expect("Failed to convert private key to PEM");
    let pub_key_pem = rsa.public_key_to_pem().expect("Failed to convert public key to PEM");
    let priv_filename = format!("rsa-{}-private.key", get_timestamp());
    let pub_filename = format!("rsa-{}-public.key", get_timestamp());
    fs::write(&priv_filename, priv_key_pem).expect("Unable to write private key to file");
    fs::write(&pub_filename, pub_key_pem).expect("Unable to write public key to file");
    priv_filename
}

#[wasm_bindgen]
pub fn encrypt_rsa(data: &str, key_file: &str) -> String {
    let key_pem = fs::read(key_file).expect("Unable to read key file");
    let rsa = Rsa::private_key_from_pem(&key_pem).expect("Invalid private key");
    let padding = RsaPadding::PKCS1;
    let data_bytes = data.as_bytes();
    let mut buffer = vec![0; rsa.size() as usize];
    rsa.private_encrypt(data_bytes, &mut buffer, padding).expect("Encryption failed");
    base64::encode(&buffer)
}

#[wasm_bindgen]
pub fn decrypt_rsa(data: &str, key_file: &str) -> String {
    let key_pem = fs::read(key_file).expect("Unable to read key file");
    let rsa = Rsa::public_key_from_pem(&key_pem).expect("Invalid public key");
    let padding = RsaPadding::PKCS1;
    let data_bytes = base64::decode(data).expect("Invalid base64 data");
    let mut buffer = vec![0; rsa.size() as usize];
    rsa.public_decrypt(&data_bytes, &mut buffer, padding).expect("Decryption failed");
    String::from_utf8(buffer).expect("Invalid UTF-8 data")
}

fn get_timestamp() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    format!("{}", in_ms)
}
