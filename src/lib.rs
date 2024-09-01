mod hasher;
mod heavy_hash;
mod logger;
mod pow;
mod target;
mod xoshiro;
use logger::Logger;

mod utils;
use hasher::PowHasher;
use pow::State;
use spectrex::astrobwtv3;
use target::{u256_from_compact_target, Uint256};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct MeTest(usize);
#[wasm_bindgen]
impl MeTest {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn add(&self, a: i32, b: i32) -> i32 {
        let mut sum = 0;
        for _ in 0..10000 {
            sum += a + b;
        }

        sum
    }
}

#[wasm_bindgen]
pub fn bitsToTarget(bits: u32) -> String {
    PowHasher::new(Uint256::new([1, 2, 3, 4]), 1)
        .finalize_with_nonce(1)
        .0
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>()
}

#[wasm_bindgen]
pub fn astrobwtv3_encode(input: &str) -> String {
    let hash_in: [u8; 32] = [
        88, 101, 183, 41, 212, 156, 190, 48, 230, 97, 94, 105, 177, 86, 88, 84, 60, 239, 203, 124,
        63, 32, 160, 222, 34, 141, 50, 108, 138, 16, 90, 230,
    ];
    let hash_out = astrobwtv3::astrobwtv3_hash(&hash_in);
    Logger::console_log!("hash_out: {:?}", hash_out);

    //hash to st

    let hash_str = hash_out
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>();

    hash_str
}
