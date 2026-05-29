use aes::cipher::{Block, BlockCipherEncrypt, KeyInit};
use aes::Aes128;
use anyhow::Result;
use serde::{Serialize, Serializer};

const EAPI_MAGIC: &[u8] = b"-36cd479b6b5-";
const EAPI_KEY: &[u8; 16] = b"e82ckenh8dichen8";
const HEX_LOWER: &[u8; 16] = b"0123456789abcdef";
const HEX_UPPER: &[u8; 16] = b"0123456789ABCDEF";

#[derive(Debug)]
pub struct EapiPayload {
    pub params: String,
}

impl Serialize for EapiPayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("EapiPayload", 1)?;
        state.serialize_field("params", &self.params)?;
        state.end()
    }
}

fn write_hex_lower(input: &[u8], output: &mut [u8]) {
    for (index, byte) in input.iter().enumerate() {
        output[index * 2] = HEX_LOWER[(byte >> 4) as usize];
        output[index * 2 + 1] = HEX_LOWER[(byte & 0x0f) as usize];
    }
}

fn encode_hex_upper(input: &[u8]) -> String {
    let mut output = vec![0u8; input.len() * 2];
    for (index, byte) in input.iter().enumerate() {
        output[index * 2] = HEX_UPPER[(byte >> 4) as usize];
        output[index * 2 + 1] = HEX_UPPER[(byte & 0x0f) as usize];
    }

    // safe.Hex output is ASCII by construction.
    unsafe { String::from_utf8_unchecked(output) }
}

fn eapi_encrypt_raw(path: &[u8], params: impl Serialize) -> Result<String> {
    let params_bytes = serde_json::to_vec(&params)?;

    let mut context = md5::Context::new();
    context.consume(b"nobody");
    context.consume(path);
    context.consume(b"use");
    context.consume(&params_bytes);
    context.consume(b"md5forencrypt");
    let digest = context.finalize();

    let mut sign_hex_buf = [0u8; 32];
    write_hex_lower(&digest.0, &mut sign_hex_buf);

    let raw_len =
        path.len() + EAPI_MAGIC.len() + params_bytes.len() + EAPI_MAGIC.len() + sign_hex_buf.len();
    // let pat = 16 - raw_len % 16;
    // exactly, in release mode, it maybe optimized by rustc!
    // let pad = ((!raw_len) & 15) + 1;
    let pad = 16 - (raw_len % 16);
    let total_len = raw_len + pad;

    let mut aes_src = Vec::with_capacity(total_len);
    aes_src.extend_from_slice(path);
    aes_src.extend_from_slice(EAPI_MAGIC);
    aes_src.extend_from_slice(&params_bytes);
    aes_src.extend_from_slice(EAPI_MAGIC);
    aes_src.extend_from_slice(&sign_hex_buf);
    aes_src.resize(total_len, pad as u8);

    let cipher = Aes128::new_from_slice(EAPI_KEY).expect("eapi key length must be 16");
    for chunk in aes_src.chunks_exact_mut(16) {
        let block: &mut Block<Aes128> = chunk
            .try_into()
            .expect("AES chunk length must always be 16 bytes");
        cipher.encrypt_block(block);
    }

    Ok(encode_hex_upper(&aes_src))
}

pub fn eapi_encrypt(path: &[u8], params: impl Serialize) -> Result<EapiPayload> {
    Ok(EapiPayload {
        params: eapi_encrypt_raw(path, params)?,
    })
}
