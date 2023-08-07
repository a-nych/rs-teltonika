// io_elements.rs
use deku::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, DekuRead)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Codec8IOElement1Byte {
    pub id: u8,
    pub value: u8,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, DekuRead)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Codec8IOElement2Byte {
    pub id: u8,
    pub value: u16,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, DekuRead)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Codec8IOElement4Byte {
    pub id: u8,
    pub value: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, DekuRead)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Codec8IOElement8Byte {
    pub id: u8,
    pub value: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, DekuRead)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Codec16IOElement1Byte {
    pub id: u16,
    pub value: u8,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, DekuRead)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Codec16IOElement2Byte {
    pub id: u16,
    pub value: u16,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, DekuRead)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Codec16IOElement4Byte {
    pub id: u16,
    pub value: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, DekuRead)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Codec16IOElement8Byte {
    pub id: u16,
    pub value: u64,
}
