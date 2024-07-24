use wasm_bindgen::prelude::*;
use aes::cipher::generic_array::GenericArray;
use aes::{Aes128, BlockDecrypt, BlockEncrypt, NewBlockCipher};
use base64::{decode, encode};
use rand::rngs::OsRng;
use rand::Rng;
use rsa::pkcs1::{FromRsaPrivateKey, FromRsaPublicKey, ToRsaPrivateKey, ToRsaPublicKey};
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use zeroize::Zeroizing;

#[wasm_bindgen]
pub fn generate_aes_key() -> String {
    let mut key = [0u8; 16];
    let mut rng = OsRng;
    rng.fill(&mut key);
    base64::encode(&key) // キーデータをbase64エンコードして返す
}

fn pkcs7_pad(data: &[u8], block_size: usize) -> Vec<u8> {
    let padding_len = block_size - (data.len() % block_size);
    let mut padded_data = Vec::with_capacity(data.len() + padding_len);
    padded_data.extend_from_slice(data);
    padded_data.extend(vec![padding_len as u8; padding_len]);
    padded_data
}

fn pkcs7_unpad(data: &[u8]) -> Result<Vec<u8>, &'static str> {
    if data.is_empty() {
        return Err("data is empty");
    }
    let padding_len = *data.last().unwrap() as usize;
    if padding_len == 0 || padding_len > data.len() {
        return Err("invalid padding length");
    }
    for &byte in &data[data.len() - padding_len..] {
        if byte as usize != padding_len {
            return Err("invalid padding");
        }
    }
    Ok(data[..data.len() - padding_len].to_vec())
}

#[wasm_bindgen]
pub fn encrypt_aes(plaintext: &str, key: &str) -> Result<String, JsValue> {
    let key = decode(key).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let cipher = Aes128::new(GenericArray::from_slice(&key));
    
    let padded_plaintext = pkcs7_pad(plaintext.as_bytes(), 16);
    let mut encrypted = Vec::with_capacity(padded_plaintext.len());
    
    for chunk in padded_plaintext.chunks(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.encrypt_block(&mut block);
        encrypted.extend_from_slice(&block);
    }
    
    Ok(encode(&encrypted))
}

#[wasm_bindgen]
pub fn decrypt_aes(ciphertext: &str, key: &str) -> Result<String, JsValue> {
    let key = decode(key).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let cipher = Aes128::new(GenericArray::from_slice(&key));
    
    let encrypted_data = decode(ciphertext).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let mut decrypted = Vec::with_capacity(encrypted_data.len());
    
    for chunk in encrypted_data.chunks(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.decrypt_block(&mut block);
        decrypted.extend_from_slice(&block);
    }
    
    let unpadded_plaintext = pkcs7_unpad(&decrypted).map_err(|e| JsValue::from_str(e))?;
    let plaintext = String::from_utf8(unpadded_plaintext).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(plaintext)
}

#[wasm_bindgen]
pub fn generate_rsa_key() -> Result<JsValue, JsValue> {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let priv_key_pem = Zeroizing::new(priv_key.to_pkcs1_pem().map_err(|e| JsValue::from_str(&e.to_string()))?);
    let pub_key_pem = Zeroizing::new(priv_key.to_public_key().to_pkcs1_pem().map_err(|e| JsValue::from_str(&e.to_string()))?);

    // Create a JavaScript object to return both keys
    let keys = js_sys::Object::new();
    js_sys::Reflect::set(&keys, &"public_key".into(), &JsValue::from_str(&pub_key_pem.to_string())).unwrap();
    js_sys::Reflect::set(&keys, &"private_key".into(), &JsValue::from_str(&priv_key_pem.to_string())).unwrap();

    Ok(keys.into())
}

#[wasm_bindgen]
pub fn encrypt_rsa(plaintext: &str, public_key_pem: &str) -> Result<String, JsValue> {
    let pub_key = RsaPublicKey::from_pkcs1_pem(public_key_pem).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let mut rng = OsRng;
    let enc_data = pub_key.encrypt(&mut rng, padding, plaintext.as_bytes()).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(encode(&enc_data))
}

#[wasm_bindgen]
pub fn decrypt_rsa(ciphertext: &str, private_key_pem: &str) -> Result<String, JsValue> {
    let priv_key = RsaPrivateKey::from_pkcs1_pem(private_key_pem).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let enc_data = decode(ciphertext).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let dec_data = priv_key.decrypt(padding, &enc_data).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let plaintext = String::from_utf8(dec_data).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(plaintext)
}
