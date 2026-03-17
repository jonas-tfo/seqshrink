use std::error::Error;
use std::io::BufWriter;
use std::io::Write;
use std::io::Read;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use bitvec::prelude::{BitVec, Msb0};
use bitvec::view::BitView;
use flate2::write::DeflateEncoder;
use flate2::read::DeflateDecoder;
use flate2::Compression as FlateCompression;

use crate::archive::Header;
use crate::archive::Record;
use crate::compress::{
    dna::{Dna2BitCompression, Dna3BitCompression},
    rna::{Rna2BitCompression, Rna3BitCompression},
    protein::Protein5BitCompression,
};
use crate::Compression;


pub fn save_fasta_compressed(output_path: &str, mode: u8, ids: Vec<String>, sequences: Vec<String>) -> Result<(), Box<dyn Error>> {
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    let header = Header { magic: *b"BLOK", mode: mode, num_records: sequences.len() as u32 };
    writer.write_all(&bincode::serialize(&header)?)?;

    // compressed ids length prefix then blob
    let ids_blob = compress_ids(&ids)?;
    writer.write_all(&(ids_blob.len() as u32).to_le_bytes())?;
    writer.write_all(&ids_blob)?;

    // encode each sequence and write to Record
    for seq in &sequences {
        let seq_bytes = seq.as_bytes();
        let bits: BitVec<u8, Msb0> = match mode {
            0 => Dna2BitCompression::encode(seq_bytes),
            1 => Dna3BitCompression::encode(seq_bytes),
            2 => Rna2BitCompression::encode(seq_bytes),
            3 => Rna3BitCompression::encode(seq_bytes),
            4 => Protein5BitCompression::encode(seq_bytes),
            _ => return Err("unknown mode".into()),
        };
        let record = Record { length: seq_bytes.len() as u32, data: bits.into_vec() };
        bincode::serialize_into(&mut writer, &record)?;
    }

    Ok(())
}

pub fn decode_fasta_compressed(input_path: &str) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
    let file = File::open(input_path)?;
    let mut reader = BufReader::new(file);

    let header: Header = bincode::deserialize_from(&mut reader)?;
    if &header.magic != b"BLOK" {
        return Err("Invalid compressed file".into());
    }
    let num_records = header.num_records as usize;

    // read ids blob
    let mut ids_len_bytes = [0u8; 4];
    reader.read_exact(&mut ids_len_bytes)?;
    let mut ids_blob = vec![0u8; u32::from_le_bytes(ids_len_bytes) as usize];
    reader.read_exact(&mut ids_blob)?;
    let ids = decompress_ids(&ids_blob)?;

    // decode each record back to a sequence string
    let mut sequences = Vec::with_capacity(num_records);
    for _ in 0..num_records {
        let record: Record = bincode::deserialize_from(&mut reader)?;
        let bits = record.data.view_bits::<Msb0>();
        let seq_bytes = match header.mode {
            0 => Dna2BitCompression::decode(bits, record.length as usize),
            1 => Dna3BitCompression::decode(bits, record.length as usize),
            2 => Rna2BitCompression::decode(bits, record.length as usize),
            3 => Rna3BitCompression::decode(bits, record.length as usize),
            4 => Protein5BitCompression::decode(bits, record.length as usize),
            _ => return Err("unknown mode".into()),
        };
        sequences.push(String::from_utf8(seq_bytes)?);
    }

    Ok((ids, sequences))
}


pub fn compress_ids(ids: &[String]) -> Result<Vec<u8>, Box<dyn Error>> {
    let raw = bincode::serialize(ids)?;
    let mut encoder = DeflateEncoder::new(Vec::new(), FlateCompression::default());
    encoder.write_all(&raw)?;
    Ok(encoder.finish()?)
}

pub fn decompress_ids(data: &[u8]) -> Result<Vec<String>, Box<dyn Error>> {
    let mut decoder = DeflateDecoder::new(data);
    let mut raw = Vec::new();
    decoder.read_to_end(&mut raw)?;
    Ok(bincode::deserialize(&raw)?)
}


pub fn parse_fasta(filename: &str) -> io::Result<(Vec<String>, Vec<String>)> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut ids: Vec<String> = Vec::new();
    let mut sequences: Vec<String> = Vec::new();
    // pre allocate buffer to prevent re-allocation
    let mut current_sequence = String::with_capacity(1024);

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() { continue; }

        if line.starts_with(">") {
            if !(current_sequence.is_empty()) {
                sequences.push(current_sequence.to_string());
                current_sequence.clear(); // clear for new record
            }
            let id: &str = &line[1..];
            ids.push(id.to_string());
        } else {
            let seq_part = &line;
            current_sequence.push_str(seq_part);
        }
    }
    if !(current_sequence.is_empty()) {
        sequences.push(current_sequence.to_string());
    }
    return Ok((ids, sequences))
}

pub fn write_fasta(ids: &[String], sequences: &[String], output_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    for (id, seq) in ids.iter().zip(sequences.iter()) {
        // wrap at 70 chars per line
        writeln!(writer, ">{}", id)?;
        for chunk in seq.as_bytes().chunks(70) {
            writer.write_all(chunk)?;
            writer.write_all(b"\n")?;
        }
    }
    Ok(())
}
