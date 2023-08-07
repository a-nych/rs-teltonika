use deku::prelude::*;
use crate::io_elements::*;

#[derive(Debug, PartialEq, DekuRead)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct AVLDataArrayCodec16 {
    timestamp: u64,
    priority: u8,
    longitude: i32,
    latitude: i32,
    altidute: i16,
    angle: u16,
    satellites: u8,
    speed: u16,
    event_id: u16,
    generation_type: u8,
    num_total_io: u8,
    num_1byte_io: u8,
    #[deku(count = "num_1byte_io")]
    io_elements_1byte: Vec<Codec16IOElement1Byte>,
    num_2bytes_io: u8,
    #[deku(count = "num_2bytes_io")]
    io_elements_2bytes: Vec<Codec16IOElement2Byte>,
    num_4bytes_io: u8,
    #[deku(count = "num_4bytes_io")]
    io_elements_4bytes: Vec<Codec16IOElement4Byte>,
    num_8bytes_io: u8,
    #[deku(count = "num_8bytes_io")]
    io_elements_8bytes: Vec<Codec16IOElement8Byte>,
}
