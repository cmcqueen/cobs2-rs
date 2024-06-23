#![allow(dead_code)]

fn main() -> Result<(), cobs2::Error> {
    let mut cobs_buf = [0x55_u8; 1000];
    let mut cobs_decode_buf = [0xCC_u8; 1000];
    //let data = b"";
    //let data = b"\x00";
    //let data = b"\xFF\xFF";
    let data = b"AAA\x05\x00CCC\x06";

    {
        let data_cobs = cobs2::cobs::encode_array(&mut cobs_buf, data)?;
        println!("COBS encode_array: {:X?}", data_cobs);
        let data_cobs_decoded = cobs2::cobs::decode_array(&mut cobs_decode_buf, data_cobs)?;
        println!("COBS decode_array: {:X?}", data_cobs_decoded);
    }

    // Try vector-based encode
    #[cfg(feature = "alloc")]
    {
        let data_cobs = cobs2::cobs::encode_vector(data)?;
        println!("COBS encode_vector: {:X?}", data_cobs);
        let data_cobs_decoded = cobs2::cobs::decode_vector(&data_cobs)?;
        println!("COBS decode_vector: {:X?}", data_cobs_decoded);
    }

    // Try iterator-based encode
    #[cfg(feature = "alloc")]
    {
        let in_data_vec = data.to_vec();
        let data_cobs: Vec<u8> = cobs2::cobs::encode_ref_iter(in_data_vec.iter()).collect();
        println!("COBS encode_ref_iter: {:X?}", data_cobs);
        let data_cobs: Vec<u8> = cobs2::cobs::encode_iter(in_data_vec.into_iter()).collect();
        println!("COBS encode_iter: {:X?}", data_cobs);
        let data_cobs_decoded: Vec<u8> = cobs2::cobs::decode_iter(data_cobs.clone().into_iter()).collect();
        println!("COBS decode_iter: {:X?}", data_cobs_decoded);
        let data_cobs_decoded: cobs2::Result<Vec<u8>> = cobs2::cobs::decode_result_iter(data_cobs.into_iter()).collect();
        println!("COBS decode_result_iter: {:X?}", data_cobs_decoded);
    }

    // Deliberately try decoding bad data.
    {
        let bad_cobs_encoded_data = b"\x00sAAA";
        let result = cobs2::cobs::decode_array(&mut cobs_decode_buf, bad_cobs_encoded_data);
        assert_eq!(result, Err(cobs2::Error::ZeroInEncodedData));
        let result: cobs2::Result<Vec<u8>> = cobs2::cobs::decode_result_iter(bad_cobs_encoded_data.into_iter().copied()).collect();
        assert_eq!(result, Err(cobs2::Error::ZeroInEncodedData));

        let bad_cobs_encoded_data = b"\x05AAA";
        let result = cobs2::cobs::decode_array(&mut cobs_decode_buf, bad_cobs_encoded_data);
        assert_eq!(result, Err(cobs2::Error::TruncatedEncodedData));
        let result: cobs2::Result<Vec<u8>> = cobs2::cobs::decode_result_iter(bad_cobs_encoded_data.into_iter().copied()).collect();
        assert_eq!(result, Err(cobs2::Error::TruncatedEncodedData));
    }

    // Now COBSR/R.
    {
        println!();
        let data_cobs = cobs2::cobsr::encode_array(&mut cobs_buf, data)?;
        println!("COBS/R encode_array: {:X?}", data_cobs);

        let data_cobs_decoded = cobs2::cobsr::decode_array(&mut cobs_decode_buf, data_cobs)?;
        println!("COBS/R decode_array: {:X?}", data_cobs_decoded);
    }
    #[cfg(feature = "alloc")]
    {
        let data_cobs = cobs2::cobsr::encode_vector(data)?;
        println!("COBS/R encode_vector: {:X?}", data_cobs);

        let data_cobs_decoded = cobs2::cobsr::decode_vector(&data_cobs)?;
        println!("COBS/R decode_vector: {:X?}", data_cobs_decoded);
    }

    Ok(())
}
