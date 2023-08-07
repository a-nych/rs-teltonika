#[cfg(tests)]
use super::*;

#[test]
fn test_simple_codec8_packet() {
  let test_data = hex_lit::hex!("003DCAFE0105000F33353230393330383634303336353508010000016B4F815B30010000000000000000000000000000000103021503010101425DBC000001").to_vec();

  let deku_test = crate::avl_data_packet::AVLDataPacket::try_from(test_data.as_ref()).unwrap();

  let serialized = serde_json::to_string_pretty(&deku_test).unwrap();

  println!("{}", serialized);
}

#[test]
fn test_large_codec8_packet() {
  let test_data = hex_lit::hex!("01e4cafe0128000f333532303934303839333937343634080400000163c803eb02010a2524c01d4a377d00d3012f130032421b0a4503f00150051503ef01510052005900be00c1000ab50008b60006426fd8cd3d1ece605a5400005500007300005a0000c0000007c70000000df1000059d910002d33c65300000000570000000064000000f7bf000000000000000163c803e6e8010a2530781d4a316f00d40131130031421b0a4503f00150051503ef01510052005900be00c1000ab50008b60005426fcbcd3d1ece605a5400005500007300005a0000c0000007c70000000ef1000059d910002d33b95300000000570000000064000000f7bf000000000000000163c803df18010a2536961d4a2e4f00d50134130033421b0a4503f00150051503ef01510052005900be00c1000ab50008b6000542702bcd3d1ece605a5400005500007300005a0000c0000007c70000001ef1000059d910002d33aa5300000000570000000064000000f7bf000000000000000163c8039ce2010a25d8d41d49f42c00dc0123120058421b0a4503f00150051503ef01510052005900be00c1000ab50009b60005427031cd79d8ce605a5400005500007300005a0000c0000007c700000019f1000059d910002d32505300000000570000000064000000f7bf000000000004").to_vec();

  let deku_test = crate::avl_data_packet::AVLDataPacket::try_from(test_data.as_ref()).unwrap();

  let serialized = serde_json::to_string_pretty(&deku_test).unwrap();

  println!("{}", serialized);
}
