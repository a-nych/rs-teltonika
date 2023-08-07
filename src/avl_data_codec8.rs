use deku::prelude::*;
use serde::{Serialize, Deserialize};
use crate::io_elements::*;

#[derive(Debug, PartialEq, DekuRead, Serialize, Deserialize)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct AVLDataArrayCodec8 {
    timestamp: u64,
    priority: u8,
    longitude: i32,
    latitude: i32,
    altidute: i16,
    angle: u16,
    satellites: u8,
    speed: u16,
    event_id: u8,
    num_total_io: u8,
    num_1byte_io: u8,
    #[deku(count = "num_1byte_io")]
    io_elements_1byte: Vec<Codec8IOElement1Byte>,
    num_2bytes_io: u8,
    #[deku(count = "num_2bytes_io")]
    io_elements_2bytes: Vec<Codec8IOElement2Byte>,
    num_4bytes_io: u8,
    #[deku(count = "num_4bytes_io")]
    io_elements_4bytes: Vec<Codec8IOElement4Byte>,
    num_8bytes_io: u8,
    #[deku(count = "num_8bytes_io")]
    io_elements_8bytes: Vec<Codec8IOElement8Byte>,
}