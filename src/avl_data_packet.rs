use deku::prelude::*;
use serde::{Serialize, Deserialize};
use crate::avl_data_codec8::AVLDataArrayCodec8;
//use crate::avl_data_codec16::AVLDataArrayCodec16;
use crate::codec::*;

#[derive(Debug, PartialEq, DekuRead, Serialize, Deserialize)]
#[deku(endian = "big")]
pub struct AVLDataPacket {
    pub length: u16,
    pub packet_id: u16,
    pub not_usable_byte: u8,
    pub avl_packet_id: u8,
    pub imei_length: u16,
    #[deku(
        count = "imei_length",
        map = "|v: Vec<u8>| -> Result<_, DekuError> { String::from_utf8(v).map_err(|_| DekuError::InvalidParam(\"Invalid IMEI string\".to_string())) }"
    )]
    pub imei: String,
    #[deku(
        map = "|v: u8| -> Result<_, DekuError> { 
            Codec::try_from(v).map_err(|e| DekuError::InvalidParam(e.to_string())) 
        }"
    )]
    pub codec: Codec,
    pub num_data_1: u8,
    #[deku(count = "num_data_1", cond = "*codec == Codec::C8")]
    pub avl_data_codec_8: Vec<AVLDataArrayCodec8>,
    // #[deku(count = "num_data_1", cond = "*codec == Codec::C16")]
    // pub avl_data_codec_8e: Vec<AVLDataArrayCodec16>,
    pub num_data_2: u8,
}
