#![allow(dead_code)]

fn main() {
    let mut cobs_buf: [u8; 1000] = [ 0; 1000 ];
    let mut cobs_decode_buf: [u8; 1000] = [ 0xCC; 1000 ];
    let data = b"AAA\x05\x00CCC\x06";
    let bad_cobs_encoded_data = b"\x05AAA";

    let data_cobs = cobs::cobs::encode(&mut cobs_buf, data);
    println!("COBS encode: {:X?}", data_cobs);

    let data_cobs_decoded = cobs::cobs::decode(&mut cobs_decode_buf, data_cobs);
    println!("COBS decode: {:X?}", data_cobs_decoded);

    let data_cobs_decoded = cobs::cobs::decode(&mut cobs_decode_buf, bad_cobs_encoded_data);
    println!("COBS decode bad data: {:X?}", data_cobs_decoded);

    // Now COBSR/R.
    println!("");
    let data_cobs = cobs::cobsr::encode(&mut cobs_buf, data);
    println!("COBS/R encode: {:X?}", data_cobs);

    let data_cobs_decoded = cobs::cobsr::decode(&mut cobs_decode_buf, data_cobs);
    println!("COBS/R decode: {:X?}", data_cobs_decoded);
}
