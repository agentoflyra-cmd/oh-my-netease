use aes::cipher::{block_padding::Pkcs7, BlockModeEncrypt, KeyIvInit};
use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use num_bigint::BigUint;
use num_traits::Num;
use serde::Serialize;

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;

const NONCE_BYTES: &[u8; 16] = b"0CoJUm6Qyw8W8jud";
const IV: &[u8; 16] = b"0102030405060708";
const RSA_E: &str = "010001";
const RSA_N: &str = concat!(
    "00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17",
    "a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114",
    "af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741",
    "d546b8e289dc6935b3ece0462db0a22b8e7",
);

#[derive(Debug, Serialize)]
pub struct WeapiPayload {
    pub params: String,
    #[serde(rename = "encSecKey")]
    pub enc_sec_key: String,
}

// Old version:
// fn create_secret_key() -> [u8; 16] {
//     let bytes: [u8; 16] = rand::random();
//     let hex = bytes
//         .iter()
//         .map(|byte| format!("{byte:02x}"))
//         .collect::<String>();

//     let mut key = [0u8; 16];
//     key.copy_from_slice(&hex.as_bytes()[..16]);
//     key
// }

fn create_secret_key() -> [u8; 16] {
    let bytes: [u8; 8] = rand::random();
    let mut out = [0u8; 16];
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for (i, byte) in bytes.iter().enumerate() {
        out[i * 2] = HEX[(byte >> 4) as usize];
        out[i * 2 + 1] = HEX[(byte & 0x0f) as usize];
    }

    out
}

fn aes_encrypt_to_base64_bytes(plain_text: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let cipher = Aes128CbcEnc::new(key.into(), IV.into());

    let mut buffer = vec![0u8; plain_text.len() + 16];
    buffer[..plain_text.len()].copy_from_slice(plain_text);

    let encrypted = cipher
        .encrypt_padded::<Pkcs7>(&mut buffer, plain_text.len())
        .expect("weapi aes encryption should always have enough buffer");

    // Old version:
    // let mut encoded = String::with_capacity(encrypted.len().div_ceil(3) * 4);
    // STANDARD.encode_string(encrypted, &mut encoded);
    let mut encoded = vec![0u8; encrypted.len().div_ceil(3) * 4];
    let written = STANDARD
        .encode_slice(encrypted, &mut encoded)
        .expect("precomputed base64 buffer should be large enough");
    encoded.truncate(written);
    encoded
}

fn aes_encrypt_to_base64_string(plain_text: &[u8], key: &[u8; 16]) -> String {
    let encoded = aes_encrypt_to_base64_bytes(plain_text, key);
    // safe.Base64 output is ASCII by construction.
    unsafe { String::from_utf8_unchecked(encoded) }
}

fn rsa_encrypt(secret_key: &[u8; 16]) -> Result<String> {
    // Old version:
    // let mut reversed = secret_key.to_vec();
    // reversed.reverse();
    // let message = BigUint::from_bytes_be(&reversed);
    let mut reversed = [0u8; 16];
    for (dst, src) in reversed.iter_mut().zip(secret_key.iter().rev()) {
        *dst = *src;
    }

    let message = BigUint::from_bytes_be(&reversed);
    let exponent = BigUint::from_str_radix(RSA_E, 16)?;
    let modulus = BigUint::from_str_radix(RSA_N, 16)?;
    let encrypted = message.modpow(&exponent, &modulus);

    Ok(format!("{encrypted:0>256x}"))
}

fn encrypt_request_with_key(data: impl Serialize, secret_key: [u8; 16]) -> Result<WeapiPayload> {
    let serialized = serde_json::to_vec(&data)?;
    // Old version:
    // let first_pass = aes_encrypt(&serialized, NONCE.as_bytes().try_into().unwrap());
    // let first_pass = aes_encrypt(&serialized, NONCE_BYTES);
    // let params = aes_encrypt(first_pass.as_bytes(), &secret_key);
    let first_pass = aes_encrypt_to_base64_bytes(&serialized, NONCE_BYTES);
    let params = aes_encrypt_to_base64_string(&first_pass, &secret_key);
    let enc_sec_key = rsa_encrypt(&secret_key)?;

    Ok(WeapiPayload {
        params,
        enc_sec_key,
    })
}

pub fn encrypt_request(data: impl Serialize) -> Result<WeapiPayload> {
    encrypt_request_with_key(data, create_secret_key())
}

#[cfg(test)]
mod tests {
    use super::{encrypt_request_with_key, rsa_encrypt, WeapiPayload};
    use anyhow::Result;
    use serde_json::json;

    #[test]
    fn rsa_output_has_fixed_width() -> Result<()> {
        let encrypted = rsa_encrypt(b"1234567890abcdef")?;
        assert_eq!(encrypted.len(), 256);
        Ok(())
    }

    #[test]
    fn encrypt_request_returns_complete_form_payload() -> Result<()> {
        let payload: WeapiPayload = encrypt_request_with_key(
            json!({
                "userId": 42,
            }),
            *b"0123456789abcdef",
        )?;

        assert!(!payload.params.is_empty());
        assert_eq!(payload.enc_sec_key.len(), 256);
        Ok(())
    }
}
