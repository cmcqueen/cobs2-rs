#![allow(dead_code)]

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut cobs_buf = [ 0x55_u8; 1000 ];
    let mut cobs_decode_buf = [ 0xCC_u8; 1000 ];
    //let data = b"";
    //let data = b"\x00";
    //let data = b"\xFF\xFF";
    let data = b"AAA\x05\x00CCC\x06";

    {
        let data_cobs = cobs::cobs::encode_array(&mut cobs_buf, data)?;
        println!("COBS encode_array: {:X?}", data_cobs);
        let data_cobs_decoded = cobs::cobs::decode_array(&mut cobs_decode_buf, data_cobs)?;
        println!("COBS decode_array: {:X?}", data_cobs_decoded);
    }

    // Try vector-based encode
    {
        let data_cobs = cobs::cobs::encode_vector(data)?;
        println!("COBS encode_vector: {:X?}", data_cobs);
        let data_cobs_decoded = cobs::cobs::decode_array(&mut cobs_decode_buf, &data_cobs)?;
        println!("COBS decode_array: {:X?}", data_cobs_decoded);
    }

    // Deliberately try decoding bad data.
    {
        let bad_cobs_encoded_data = b"\x00sAAA";
        let result = cobs::cobs::decode_array(&mut cobs_decode_buf, bad_cobs_encoded_data);
        assert_eq!(result, Err(cobs::Error::ZeroInEncodedData));

        let bad_cobs_encoded_data = b"\x05AAA";
        let result = cobs::cobs::decode_array(&mut cobs_decode_buf, bad_cobs_encoded_data);
        assert_eq!(result, Err(cobs::Error::TruncatedEncodedData));
    }

    // Now COBSR/R.
    {
        println!("");
        let data_cobs = cobs::cobsr::encode_array(&mut cobs_buf, data)?;
        println!("COBS/R encode_array: {:X?}", data_cobs);

        let data_cobs_decoded = cobs::cobsr::decode_array(&mut cobs_decode_buf, data_cobs)?;
        println!("COBS/R decode_array: {:X?}", data_cobs_decoded);
    }
    {
        let data_cobs = cobs::cobsr::encode_vector(data)?;
        println!("COBS/R encode_vector: {:X?}", data_cobs);

        let data_cobs_decoded = cobs::cobsr::decode_array(&mut cobs_decode_buf, &data_cobs)?;
        println!("COBS/R decode_array: {:X?}", data_cobs_decoded);
    }

    Ok(())
}
