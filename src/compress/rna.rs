
use bitvec::prelude::{BitVec, Msb0, BitSlice};
use crate::Compression;
use crate::compress::{encode_n_bit, decode_n_bit};

const MAP_2BIT_RNA: [u8; 256] = {
    let mut map = [0u8; 256];
    map[b'C' as usize] = 1; map[b'c' as usize] = 1;
    map[b'G' as usize] = 2; map[b'g' as usize] = 2;
    map[b'U' as usize] = 3; map[b'u' as usize] = 3;
    map
};
const INVERSE_2BIT_RNA: [u8; 4] = [b'A', b'C', b'G', b'U'];

const MAP_3BIT_RNA: [u8; 256] = {
    let mut map = [4u8; 256];
    map[b'A' as usize] = 0; map[b'a' as usize] = 0;
    map[b'C' as usize] = 1; map[b'c' as usize] = 1;
    map[b'G' as usize] = 2; map[b'g' as usize] = 2;
    map[b'U' as usize] = 3; map[b'u' as usize] = 3;
    map[b'N' as usize] = 4; map[b'n' as usize] = 4;
    map[b'T' as usize] = 5; map[b't' as usize] = 5;
    map[b'-' as usize] = 6;
    map
};
const INVERSE_3BIT_RNA: [u8; 8] = [b'A', b'C', b'G', b'U', b'N', b'T', b'-', b'?'];

pub struct Rna2BitCompression;
pub struct Rna3BitCompression;

impl Compression for Rna2BitCompression {
    fn encode(sequence: &[u8]) -> BitVec<u8, Msb0> { encode_n_bit(sequence, &MAP_2BIT_RNA, 2) }
    fn decode(bits: &BitSlice<u8, Msb0>, len: usize) -> Vec<u8> { decode_n_bit(bits, len, &INVERSE_2BIT_RNA, 2) }
}

impl Compression for Rna3BitCompression {
    fn encode(sequence: &[u8]) -> BitVec<u8, Msb0> { encode_n_bit(sequence, &MAP_3BIT_RNA, 3) }
    fn decode(bits: &BitSlice<u8, Msb0>, len: usize) -> Vec<u8> { decode_n_bit(bits, len, &INVERSE_3BIT_RNA, 3) }
}
