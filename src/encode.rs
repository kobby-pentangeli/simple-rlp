//! Simple RLP encoding library

use borsh::{self, BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

/// RLP encoding struct
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(crate = "serde")]
pub struct RlpEncode(Vec<u8>);

impl RlpEncode {
    /// Starting point for short u8
    pub const LIST_SHORT_START: u8 = 0xc0;
    /// Starting point for long u8
    pub const LIST_LONG_START: u8 = 0xf7;

    const MAX_UINT8: u8 = u8::MAX;
    const MAX_UINT16: u16 = u16::MAX;
    const MAX_UINT32: u32 = u32::MAX;
    const MAX_UINT64: u64 = u64::MAX;
    const MAX_UINT128: u128 = u128::MAX;

    /// RLP-encode a byte string
    /// Returns the RLP encoded string in bytes
    fn encode_bytes(bytes: Self) -> Vec<u8> {
        let encoded: Vec<u8>;
        if bytes.0.len() == 1 && bytes.0[0] <= 128 {
            encoded = bytes.0.clone();
        } else {
            encoded = Self::concat(&Self::encode_length(bytes.0.len(), 128), &bytes.0);
        }
        encoded
    }

    /// RLP-encode a list of RLP encoded byte strings
    /// Returns the RLP encoded list of items in bytes
    fn encode_list(bytes: Vec<Self>) -> Vec<u8> {
        todo!()
    }

    /// RLP-encode a uint
    /// Returns the RLP encoded uint in bytes
    fn encode_uint() -> Vec<u8> {
        todo!()
    }

    fn encode_int() -> Vec<u8> {
        todo!()
    }

    fn encode_bool() -> Vec<u8> {
        todo!()
    }

    fn encode_length(len: usize, offset: usize) -> Vec<u8> {
        todo!()
    }

    fn to_binary(x: usize) -> Vec<u8> {
        todo!()
    }

    fn memcpy(dest: usize, src: usize, len: usize) {
        todo!()
    }

    fn flatten(list: Vec<Vec<u8>>) -> Vec<u8> {
        todo!()
    }

    fn concat(before: &[u8], after: &[u8]) -> Vec<u8> {
        todo!()
    }

    fn add_length(length: usize, is_long_list: bool) -> Vec<u8> {
        todo!()
    }

    fn encode_uint_by_length(length: usize) -> Vec<u8> {
        todo!()
    }

    fn bit_length(n: usize) -> usize {
        todo!()
    }
}
