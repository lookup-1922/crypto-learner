use num::Integer;
use std::fs::File;
use std::io::{Read, Write};
use std::string;
use std::time::{SystemTime, UNIX_EPOCH};
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

// 最小公倍数を求める
fn gcd(a: u64, b: u64) -> u64 {
    let mut x = a;
    let mut y = b;

    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }

    x
}

#[wasm_bindgen]
pub struct KeyPair {
    pub e: u64,
    pub d: u64,
}

impl KeyPair {
    #[wasm_bindgen(constructor)]
    pub fn generate_aes_key(e: u64, d: u64) -> KeyPair {
        KeyPair { e, d }
    }
}

// AES鍵生成
#[wasm_bindgen]
pub fn generate_aes_key(p: u64, q: u64) -> KeyPair {
    let n = p * q;
    let l = (p - 1) * (q - 1);

    // E を求める
    let mut e = 2;
    while e < l {
        if gcd(e, l) == 1 {
            break;
        }
        e += 1;
    }

    // D を求める
    let d = (2..l).find(|d| (e * d) % l == 1).unwrap();

    KeyPair::new(e, d)
}

// AES暗号化
#[wasm_bindgen]
pub fn encrypt_aes(data: &str, key_file: &str) -> String {}

// AES復号
#[wasm_bindgen]
pub fn decrypt_aes(data: &str, key_file: &str) -> String {}

// RSA鍵生成
#[wasm_bindgen]
pub fn generate_rsa_key() -> String {}

// RSA暗号化
#[wasm_bindgen]
pub fn encrypt_rsa(data: &str, key_file: &str) -> String {}

// RSA復号化
#[wasm_bindgen]
pub fn decrypt_rsa(data: &str, key_file: &str) -> String {}
