
use bitvec::prelude::{BitVec, Msb0, BitSlice};
use crate::Compression;
use crate::compress::{encode_n_bit, decode_n_bit};

const MAP_5BIT_PROT: [u8; 256] = {
    let mut map = [23u8; 256]; // Default to 23/X
    map[b'A' as usize] = 0;  map[b'a' as usize] = 0;
    map[b'R' as usize] = 1;  map[b'r' as usize] = 1;
    map[b'N' as usize] = 2;  map[b'n' as usize] = 2;
    map[b'D' as usize] = 3;  map[b'd' as usize] = 3;
    map[b'C' as usize] = 4;  map[b'c' as usize] = 4;
    map[b'Q' as usize] = 5;  map[b'q' as usize] = 5;
    map[b'E' as usize] = 6;  map[b'e' as usize] = 6;
    map[b'G' as usize] = 7;  map[b'g' as usize] = 7;
    map[b'H' as usize] = 8;  map[b'h' as usize] = 8;
    map[b'I' as usize] = 9;  map[b'i' as usize] = 9;
    map[b'L' as usize] = 10; map[b'l' as usize] = 10;
    map[b'K' as usize] = 11; map[b'k' as usize] = 11;
    map[b'M' as usize] = 12; map[b'm' as usize] = 12;
    map[b'F' as usize] = 13; map[b'f' as usize] = 13;
    map[b'P' as usize] = 14; map[b'p' as usize] = 14;
    map[b'S' as usize] = 15; map[b's' as usize] = 15;
    map[b'T' as usize] = 16; map[b't' as usize] = 16;
    map[b'W' as usize] = 17; map[b'w' as usize] = 17;
    map[b'Y' as usize] = 18; map[b'y' as usize] = 18;
    map[b'V' as usize] = 19; map[b'v' as usize] = 19;
    // special
    map[b'B' as usize] = 20; map[b'b' as usize] = 20;
    map[b'Z' as usize] = 21; map[b'z' as usize] = 21;
    map[b'J' as usize] = 22; map[b'j' as usize] = 22;
    map[b'X' as usize] = 23; map[b'x' as usize] = 23;
    map[b'*' as usize] = 24;
    map[b'-' as usize] = 25;
    map[b'U' as usize] = 26; map[b'u' as usize] = 26;
    map[b'O' as usize] = 27; map[b'o' as usize] = 27;
    map
};

const INVERSE_5BIT_PROT: [u8; 32] = [
    b'A', b'R', b'N', b'D', b'C', b'Q', b'E', b'G',
    b'H', b'I', b'L', b'K', b'M', b'F', b'P', b'S',
    b'T', b'W', b'Y', b'V', b'B', b'Z', b'J', b'X',
    b'*', b'-', b'U', b'O', b'?', b'?', b'?', b'?'
];

pub struct Protein5BitCompression;

impl Compression for Protein5BitCompression {
    fn encode(sequence: &[u8]) -> BitVec<u8, Msb0> { encode_n_bit(sequence, &MAP_5BIT_PROT, 5) }
    fn decode(bits: &BitSlice<u8, Msb0>, len: usize) -> Vec<u8> { decode_n_bit(bits, len, &INVERSE_5BIT_PROT, 5) }
}
