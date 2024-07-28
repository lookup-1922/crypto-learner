use aes::cipher::generic_array::GenericArray;
use aes::{Aes256, BlockDecrypt, BlockEncrypt, NewBlockCipher};
use base64::{decode, encode};
use rand::rngs::OsRng;
use rand::Rng;
use rsa::pkcs1::{FromRsaPrivateKey, FromRsaPublicKey, ToRsaPrivateKey, ToRsaPublicKey};
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use wasm_bindgen::prelude::*;

// AES鍵を生成してBase64エンコードした文字列を返す関数
#[wasm_bindgen]
pub fn generate_aes_key() -> String {
    let mut key = [0u8; 32];
    let mut rng = OsRng;
    rng.fill(&mut key);
    base64::encode(&key)
}

// AESで暗号化する関数
#[wasm_bindgen]
pub fn encrypt_aes(plaintext: &str, key: &str) -> Result<String, JsValue> {
    // Base64でエンコードされた鍵をデコード
    let key = decode(key).map_err(|e: base64::DecodeError| JsValue::from_str(&e.to_string()))?;
    let cipher = Aes256::new(GenericArray::from_slice(&key));

    // 平文をPKCS7パディング
    let padded_plaintext = pkcs7_pad(plaintext.as_bytes(), 16);
    let mut encrypted = Vec::with_capacity(padded_plaintext.len());

    // パディングされた平文をブロック単位で暗号化
    for chunk in padded_plaintext.chunks(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.encrypt_block(&mut block);
        encrypted.extend_from_slice(&block);
    }

    // 暗号化されたデータをBase64エンコードして返す
    Ok(encode(&encrypted))
}

// AESで復号する関数
#[wasm_bindgen]
pub fn decrypt_aes(ciphertext: &str, key: &str) -> Result<String, JsValue> {
    // Base64でエンコードされた鍵をデコード
    let key = decode(key).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let cipher = Aes256::new(GenericArray::from_slice(&key));

    // 暗号化されたデータをデコード
    let encrypted_data = decode(ciphertext).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let mut decrypted = Vec::with_capacity(encrypted_data.len());

    // 暗号化されたデータをブロック単位で復号
    for chunk in encrypted_data.chunks(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.decrypt_block(&mut block);
        decrypted.extend_from_slice(&block);
    }

    // 復号されたデータのPKCS7パディングを除去
    let unpadded_plaintext = pkcs7_unpad(&decrypted).map_err(|e| JsValue::from_str(e))?;
    let plaintext =
        String::from_utf8(unpadded_plaintext).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(plaintext)
}

// PKCS7パディングを追加する関数
fn pkcs7_pad(data: &[u8], block_size: usize) -> Vec<u8> {
    let padding_len = block_size - (data.len() % block_size);
    let mut padded_data = Vec::with_capacity(data.len() + padding_len);
    padded_data.extend_from_slice(data);
    padded_data.resize(padded_data.len() + padding_len, padding_len as u8);
    padded_data
}

// PKCS7パディングを除去する関数
fn pkcs7_unpad(data: &[u8]) -> Result<Vec<u8>, &'static str> {
    if data.is_empty() {
        return Err("data is empty");
    }
    let padding_len = data.last().copied().unwrap_or(0) as usize;
    if padding_len == 0
        || padding_len > data.len()
        || data[data.len() - padding_len..]
            .iter()
            .any(|&byte| byte as usize != padding_len)
    {
        return Err("invalid padding length");
    }
    Ok(data[..data.len() - padding_len].to_vec())
}

// RSA鍵ペアを生成してJavaScriptオブジェクトとして返す関数
#[wasm_bindgen]
pub fn generate_rsa_key() -> Result<JsValue, JsValue> {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key =
        RsaPrivateKey::new(&mut rng, bits).map_err(|e| JsValue::from_str(&e.to_string()))?;

    // 秘密鍵と公開鍵をPEM形式でエンコード
    let priv_key_pem = priv_key
        .to_pkcs1_pem()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let pub_key_pem = priv_key
        .to_public_key()
        .to_pkcs1_pem()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // 鍵をJavaScriptオブジェクトにセットして返す
    let keys = js_sys::Object::new();
    js_sys::Reflect::set(
        &keys,
        &"public_key".into(),
        &JsValue::from_str(&pub_key_pem),
    )
    .unwrap();
    js_sys::Reflect::set(
        &keys,
        &"private_key".into(),
        &JsValue::from_str(&priv_key_pem),
    )
    .unwrap();

    Ok(keys.into())
}

// RSAで暗号化する関数
#[wasm_bindgen]
pub fn encrypt_rsa(plaintext: &str, public_key_pem: &str) -> Result<String, JsValue> {
    // 公開鍵をPEM形式から読み込む
    let pub_key = RsaPublicKey::from_pkcs1_pem(public_key_pem)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let mut rng = OsRng;

    // 平文を公開鍵で暗号化してBase64エンコードして返す
    let enc_data = pub_key
        .encrypt(&mut rng, padding, plaintext.as_bytes())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(encode(&enc_data))
}

// RSAで復号する関数
#[wasm_bindgen]
pub fn decrypt_rsa(ciphertext: &str, private_key_pem: &str) -> Result<String, JsValue> {
    // 秘密鍵をPEM形式から読み込む
    let priv_key = RsaPrivateKey::from_pkcs1_pem(private_key_pem)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let padding = PaddingScheme::new_pkcs1v15_encrypt();

    // 暗号化されたデータをデコードして秘密鍵で復号
    let enc_data = decode(ciphertext).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let dec_data = priv_key
        .decrypt(padding, &enc_data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // 復号されたデータをUTF-8文字列に変換して返す
    let plaintext = String::from_utf8(dec_data).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(plaintext)
}
