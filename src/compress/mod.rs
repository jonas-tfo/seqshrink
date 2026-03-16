pub mod dna;
pub mod rna;
pub mod protein;

use bitvec::{field::BitField, prelude::{BitSlice, BitVec, Msb0}, view::BitView};

pub fn encode_n_bit(sequence: &[u8], map: &[u8; 256], n: usize) -> BitVec<u8, Msb0> {
  let mut bits = BitVec::new();
  for &value in sequence {
      bits.extend_from_bitslice(&map[value as usize].view_bits::<Msb0>()[(8 - n)..8]);
  }
  bits
}

pub fn decode_n_bit(bits: &BitSlice<u8, Msb0>, len: usize, inverse: &[u8], n: usize) -> Vec<u8> {
  let mut output = Vec::with_capacity(len); // dont need to re allocate when len known
  for chunk in bits.chunks_exact(n).take(len) {
      let value: u8 = chunk.load_be();
      output.push(inverse[value as usize]);
  }
  output
}

