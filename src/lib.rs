use std::fs::File;
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use aes::cipher::generic_array::GenericArray;
use aes::{Aes128, BlockDecrypt, BlockEncrypt, NewBlockCipher};
use base64::{decode, encode};
use rand::rngs::OsRng;
use rand::Rng;
use rsa::pkcs1::{FromRsaPrivateKey, FromRsaPublicKey, ToRsaPrivateKey, ToRsaPublicKey};
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use wasm_bindgen::prelude::*;

// 現在のタイムスタンプを取得するヘルパー関数
fn get_timestamp() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms =
        since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    format!("{}", in_ms)
}

// ファイルにバイト列を書き込むヘルパー関数
fn write_to_file(filename: &str, data: &[u8]) {
    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(data).expect("Unable to write data");
}

// ファイルからバイト列を読み込むヘルパー関数
fn read_from_file(filename: &str, buffer: &mut [u8]) {
    let mut file = File::open(filename).expect("Unable to open file");
    file.read_exact(buffer).expect("Unable to read data");
}

// AES鍵生成
#[wasm_bindgen]
pub fn generate_aes_key() -> String {
    let mut key = [0u8; 16];
    let mut rng = OsRng;
    rng.fill(&mut key);
    base64::encode(&key) // キーデータをbase64エンコードして返す
}

// AES暗号化
#[wasm_bindgen]
pub fn encrypt_aes(data: &str, key_file: &str) -> String {
    let mut key = [0u8; 16];
    read_from_file(key_file, &mut key);
    let cipher = Aes128::new(GenericArray::from_slice(&key));
    let mut block = GenericArray::clone_from_slice(&data.as_bytes()[..16]);
    cipher.encrypt_block(&mut block);
    encode(&block)
}

// AES復号化
#[wasm_bindgen]
pub fn decrypt_aes(data: &str, key_file: &str) -> String {
    let mut key = [0u8; 16];
    read_from_file(key_file, &mut key);
    let cipher = Aes128::new(GenericArray::from_slice(&key));
    let encrypted_data = decode(data).expect("Invalid base64 data");
    let mut block = GenericArray::clone_from_slice(&encrypted_data);
    cipher.decrypt_block(&mut block);
    String::from_utf8(block.to_vec()).expect("Invalid UTF-8 data")
}

// RSA鍵生成
#[wasm_bindgen]
pub fn generate_rsa_key() -> String {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("Unable to generate private key");
    let pub_key = RsaPublicKey::from(&priv_key);
    let priv_key_pem = priv_key
        .to_pkcs1_pem()
        .expect("Unable to convert private key to PEM");
    let pub_key_pem = pub_key
        .to_pkcs1_pem()
        .expect("Unable to convert public key to PEM");
    let filename_priv = format!("rsa-{}-private.key", get_timestamp());
    let filename_pub = format!("rsa-{}-public.key", get_timestamp());
    write_to_file(&filename_priv, priv_key_pem.as_bytes());
    write_to_file(&filename_pub, pub_key_pem.as_bytes());
    filename_priv
}

// RSA暗号化
#[wasm_bindgen]
pub fn encrypt_rsa(data: &str, key_file: &str) -> String {
    let mut pem = String::new();
    File::open(key_file)
        .expect("Unable to open file")
        .read_to_string(&mut pem)
        .expect("Unable to read key file");
    let pub_key = RsaPublicKey::from_pkcs1_pem(&pem).expect("Invalid public key");
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let mut rng = OsRng;
    let enc_data = pub_key
        .encrypt(&mut rng, padding, data.as_bytes())
        .expect("Encryption failed");
    encode(&enc_data)
}

// RSA復号化
#[wasm_bindgen]
pub fn decrypt_rsa(data: &str, key_file: &str) -> String {
    let mut pem = String::new();
    File::open(key_file)
        .expect("Unable to open file")
        .read_to_string(&mut pem)
        .expect("Unable to read key file");
    let priv_key = RsaPrivateKey::from_pkcs1_pem(&pem).expect("Invalid private key");
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let enc_data = decode(data).expect("Invalid base64 data");
    let dec_data = priv_key
        .decrypt(padding, &enc_data)
        .expect("Decryption failed");
    String::from_utf8(dec_data).expect("Invalid UTF-8 data")
}
