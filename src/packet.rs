// main.rs
use crate::io_elements::{IOElement1Byte, IOElement2Byte, IOElement4Byte, IOElement8Byte};
use crate::error::CodecError;
use deku::prelude::*;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
pub enum Codec {
    C8,
    C8E,
    C16
}

impl TryFrom<u8> for Codec {
    type Error = CodecError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x08 => Ok(Codec::C8),
            0x8E => Ok(Codec::C8E),
            0x10 => Ok(Codec::C16),
            _ => Err(CodecError::UnsupportedValue(value)),
        }
    }
}


#[derive(Debug, PartialEq, DekuRead)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct AVLDataArray {
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
    io_elements_1byte: Vec<IOElement1Byte>,
    num_2bytes_io: u8,
    #[deku(count = "num_2bytes_io")]
    io_elements_2bytes: Vec<IOElement2Byte>,
    num_4bytes_io: u8,
    #[deku(count = "num_4bytes_io")]
    io_elements_4bytes: Vec<IOElement4Byte>,
    num_8bytes_io: u8,
    #[deku(count = "num_8bytes_io")]
    io_elements_8bytes: Vec<IOElement8Byte>,
}

#[derive(Debug, PartialEq, DekuRead)]
#[deku(endian = "big")]
pub struct AVLDataPacket {
    pub length: u16, // packet length (excluding this field) in big ending byte order.
    pub packet_id: u16, // packet ID unique for this channel. 
    pub not_usable_byte: u8, // 
    pub avl_packet_id: u8, // 
    pub imei_length: u16, //
    #[deku(
        count = "imei_length",
        map = "|v: Vec<u8>| -> Result<_, DekuError> { String::from_utf8(v).map_err(|_| DekuError::InvalidParam(\"Invalid UTF-8 string\".to_string())) }"
    )]
    pub imei: String,
    #[deku(
        map = "|v: u8| -> Result<_, DekuError> { 
            Codec::try_from(v).map_err(|e| DekuError::InvalidParam(e.to_string())) 
        }"
    )]
    pub codec: Codec,
    pub num_data_1: u8,
    #[deku(count = "num_data_1")]
    pub avl_data: Vec<AVLDataArray>,
    pub num_data_2: u8,
}

impl AVLDataPacket {
    pub fn check_num_data(&self) -> Result<(), DekuError> {
        if self.num_data_1 == self.num_data_2 {
            Ok(())
        } else {
            Err(DekuError::InvalidParam("num_data_1 and num_data_2 are not equal".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_lit::hex;

    #[test]
    fn test_avl_data_packet_parsing() {
        let test_data = hex!("003DCAFE0105000F33353230393330383634303336353508010000016B4F815B30010000000000000000000000000000000103021503010101425DBC000001").to_vec();

        let deku_test = AVLDataPacket::try_from(test_data.as_ref()).unwrap();

        assert_eq!(
            AVLDataPacket {
                length: 0x003D,
                packet_id: 0xCAFE,
                not_usable_byte: 0x01,
                avl_packet_id: 0x05,
                imei_length: 0x000F,
                imei: "352093086403655".to_string(),
                //imei: vec![0x33,0x35,0x32,0x30,0x39,0x33,0x30,0x38,0x36,0x34,0x30,0x33,0x36,0x35,0x35],
                codec: Codec::C8,
                num_data_1: 0x01,
                avl_data: vec![AVLDataArray{
                    timestamp: 0x0000016B4F815B30,
                    priority: 0x01,
                    longitude: 0x00000000,
                    latitude: 0x00000000,
                    altidute: 0x0000,
                    angle: 0x0000,
                    satellites: 0x00,
                    speed: 0x00,
                    event_id: 0x01,
                    num_total_io: 0x03,
                    num_1byte_io: 0x02,
                    io_elements_1byte: vec![IOElement1Byte{
                        id: 0x15,
                        value: 0x03
                    }, IOElement1Byte{
                        id: 0x01,
                        value: 0x01
                    }],
                    num_2bytes_io: 0x01,
                    io_elements_2bytes: vec![IOElement2Byte{
                        id: 0x42,
                        value: 0x5DBC
                    }],
                    num_4bytes_io: 0x00,
                    io_elements_4bytes: vec![],
                    num_8bytes_io: 0x00,
                    io_elements_8bytes: vec![]
                }],
                num_data_2: 0x01,
            },
            deku_test
        );

        assert!(deku_test.check_num_data().is_ok());

        let raw_packet_size = test_data.len() - 2; // Exclude the 'length' field
        assert_eq!(raw_packet_size, deku_test.length as usize);
    }
}
