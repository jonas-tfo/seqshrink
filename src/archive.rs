use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub magic: [u8; 4], // b"BLOK" needs this to be valid compressed file
    pub mode: u8, // 0 DNA2, 1 DNA3, 2 RNA2, 3 RNA3, 4 PROT5
    pub num_records: u32, // original length
}

#[derive(Serialize, Deserialize)]
pub struct Record {
    pub data: Vec<u8>,
    pub length: u32
}
