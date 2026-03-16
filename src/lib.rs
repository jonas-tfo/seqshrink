use bitvec::prelude::*;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub enum Alphabet {
    Dna2Bit,      // A, C, G, T
    Dna3Bit,      // A, C, G, T, N, Gap
    Protein5Bit,  // 20 AA, X, Stop
}

pub trait Compression {
    // ascii to bits
    fn encode(sequence: &[u8]) -> BitVec<u8, Msb0>;
    // bits to ascii
    fn decode(bits: &BitSlice<u8, Msb0>, len: usize) -> Vec<u8>;
}

pub mod compress;
pub mod archive;
pub mod utils;
