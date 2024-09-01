use crate::logger::Logger;
use crate::{
    hasher::{HeaderHasher, PowHasher},
    heavy_hash::Matrix,
    target::{u256_from_compact_target, Uint256},
};
use spectrex::astrobwtv3;
use std::convert::TryInto;
use std::fmt::Error;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
#[wasm_bindgen]
pub struct State {
    #[allow(dead_code)]
    pub id: usize,
    matrix: Matrix,
    pub nonce: u64,
    // PRE_POW_HASH || TIME || 32 zero byte padding; without NONCE
    hasher: PowHasher,
}
#[wasm_bindgen]
impl State {
    #[inline]
    pub fn new(id: usize, pre_pow_hash: &[u8], timestamp: u64) -> Self {
        // PRE_POW_HASH || TIME || 32 zero byte padding || NONCE
        let mut pre_pow_hash_fixed = [0u8; 32];
        pre_pow_hash_fixed.copy_from_slice(pre_pow_hash);
        // Logger::console_log!("hash_out: {:?}", pre_pow_hash_fixed);
        // Logger::console_log!("timestamp: {:?}", timestamp);

        let hasher = PowHasher::new(Uint256::from_le_bytes(pre_pow_hash_fixed), timestamp);
        let matrix = Matrix::generate(Uint256::from_le_bytes(pre_pow_hash_fixed));

        Self {
            id,
            matrix,
            nonce: 0,
            hasher,
        }
    }

    #[inline(always)]
    // PRE_POW_HASH || TIME || 32 zero byte padding || NONCE
    pub fn calculate_pow(&self, nonce: u64) -> String {
        // Hasher already contains PRE_POW_HASH || TIME || 32 zero byte padding; so only the NONCE is missing
        //Logger::console_log!("nonce: {:?}", nonce);
        let hash = self.hasher.finalize_with_nonce(nonce);
        // Logger::console_log!(
        //     "cshake hash: {:?}",
        //     hash.to_le_bytes()
        //         .iter()
        //         .map(|x| format!("{:02x}", x))
        //         .collect::<String>()
        // );
        let bwt_hash = astrobwtv3::astrobwtv3_hash(&hash.to_le_bytes());
        // Logger::console_log!("bwt_hash: {:?}", bwt_hash);
        let mut le_bytes = self
            .matrix
            .heavy_hash(Uint256::from_le_bytes(bwt_hash))
            .to_le_bytes();

        le_bytes.reverse();

        le_bytes
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>()
    }
}

#[cfg(not(any(target_pointer_width = "64", target_pointer_width = "32")))]
compile_error!("Supporting only 32/64 bits");

#[allow(dead_code)] // False Positive: https://github.com/rust-lang/rust/issues/88900
#[derive(Debug)]
enum FromHexError {
    OddLength,
    InvalidStringLength,
    InvalidHexCharacter { c: char, index: usize },
}

#[inline(always)]
fn decode_to_slice<T: AsRef<[u8]>>(data: T, out: &mut [u8]) -> Result<(), FromHexError> {
    let data = data.as_ref();
    if data.len() % 2 != 0 {
        return Err(FromHexError::OddLength);
    }
    if data.len() / 2 != out.len() {
        return Err(FromHexError::InvalidStringLength);
    }

    for (i, byte) in out.iter_mut().enumerate() {
        *byte = val(data[2 * i], 2 * i)? << 4 | val(data[2 * i + 1], 2 * i + 1)?;
    }

    #[inline(always)]
    fn val(c: u8, idx: usize) -> Result<u8, FromHexError> {
        match c {
            b'A'..=b'F' => Ok(c - b'A' + 10),
            b'a'..=b'f' => Ok(c - b'a' + 10),
            b'0'..=b'9' => Ok(c - b'0'),
            _ => Err(FromHexError::InvalidHexCharacter {
                c: c as char,
                index: idx,
            }),
        }
    }

    Ok(())
}
