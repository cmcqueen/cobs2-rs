use ::cobs::{cobs, cobsr};

struct DataEncodedMapping<'a> {
    pub description: &'a str,
    pub rawdata: &'a [u8],
    pub encoded: &'a [u8],
}

const PREDEFINED_ENCODINGS: [DataEncodedMapping; 16] = [
    DataEncodedMapping { description: "empty",                          rawdata: b"",                      encoded: b"\x01"                        },
    DataEncodedMapping { description: "1 non-zero",                     rawdata: b"1",                     encoded: b"\x021"                       },
    DataEncodedMapping { description: "5 non-zero",                     rawdata: b"12345",                 encoded: b"\x0612345"                   },
    DataEncodedMapping { description: "1 zero in middle",               rawdata: b"12345\x006789",         encoded: b"\x0612345\x056789"           },
    DataEncodedMapping { description: "2 clumps starting with zero",    rawdata: b"\x0012345\x006789",     encoded: b"\x01\x0612345\x056789"       },
    DataEncodedMapping { description: "2 clumps ending with zero",      rawdata: b"12345\x006789\x00",     encoded: b"\x0612345\x056789\x01"       },
    DataEncodedMapping { description: "1 zero",                         rawdata: b"\x00",                  encoded: b"\x01\x01"                    },
    DataEncodedMapping { description: "2 zeros",                        rawdata: b"\x00\x00",              encoded: b"\x01\x01\x01"                },
    DataEncodedMapping { description: "3 zeros",                        rawdata: b"\x00\x00\x00",          encoded: b"\x01\x01\x01\x01"            },
    DataEncodedMapping {
        description: "253 non-zero bytes",
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123",
        encoded: b"\xFE0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123",
    },
    DataEncodedMapping {
        description: "254 non-zero bytes",
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234",
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234",
    },
    DataEncodedMapping {
        description: "255 non-zero bytes",
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst12345",
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234\x025",
    },
    DataEncodedMapping {
        description: "zero followed by 255 non-zero bytes",
        rawdata: b"\x000123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst12345",
        encoded: b"\x01\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234\x025",
    },
    DataEncodedMapping {
        description: "253 non-zero bytes followed by zero",
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123\x00",
        encoded: b"\xFE0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123\x01",
    },
    DataEncodedMapping {
        description: "254 non-zero bytes followed by zero",
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234\x00",
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234\x01\x01",
    },
    DataEncodedMapping {
        description: "255 non-zero bytes followed by zero",
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst12345\x00",
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234\x025\x01",
    },
];

/*
 * Decoding-specific tests. These are for unusual encoded data, which a correct encoder wouldn't normally generate, but
 * could be encountered from a different encoder implementation that generates non-optimal encodings.
 */
const PREDEFINED_DECODINGS: [DataEncodedMapping; 2] = [
    // Handle an empty string, returning an empty string.
    DataEncodedMapping { description: "empty", rawdata: b"",                      encoded: b""                            },
    DataEncodedMapping {
        description: "254 non-zero bytes",
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234",
        // A naive encoder implementation might not handle this edge case optimally, and append a redundant trailing \x01.
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234\x01",
    },
];

#[test]
fn test_cobs_encode_max_output_size() {
    assert_eq!(1, cobs::encode_max_output_size(0));
    assert_eq!(2, cobs::encode_max_output_size(1));
    assert_eq!(3, cobs::encode_max_output_size(2));

    assert_eq!(254, cobs::encode_max_output_size(253));
    assert_eq!(255, cobs::encode_max_output_size(254));
    assert_eq!(257, cobs::encode_max_output_size(255));
    assert_eq!(258, cobs::encode_max_output_size(256));

    assert_eq!(509, cobs::encode_max_output_size(507));
    assert_eq!(510, cobs::encode_max_output_size(508));
    assert_eq!(512, cobs::encode_max_output_size(509));
    assert_eq!(513, cobs::encode_max_output_size(510));

    assert_eq!(
        usize::max_value(),
        cobs::encode_max_output_size(usize::max_value())
    );
    let increase = usize::max_value() / 255;
    assert_eq!(
        usize::max_value(),
        cobs::encode_max_output_size(usize::max_value() - increase)
    );
}

#[test]
fn test_cobs_array_predefined() {
    for mapping in PREDEFINED_ENCODINGS.iter() {
        let mut encode_out_vec = vec![0_u8; cobs::encode_max_output_size(mapping.rawdata.len())];
        let enc_result = cobs::encode_array(&mut encode_out_vec[..], mapping.rawdata);
        assert!(enc_result.is_ok());
        assert_eq!(
            enc_result.clone().unwrap(),
            mapping.encoded,
            "{}",
            mapping.description
        );

        let mut decode_out_vec =
            vec![0_u8; cobs::decode_max_output_size(enc_result.clone().unwrap().len())];
        let dec_result = cobs::decode_array(&mut decode_out_vec[..], &enc_result.clone().unwrap());
        assert!(dec_result.is_ok());
        assert_eq!(
            dec_result.unwrap(),
            mapping.rawdata,
            "{}",
            mapping.description
        );

        // COBS/R decode function should also be able to decode COBS-encoded rawdata.
        let mut decode_out_vec =
            vec![0_u8; cobsr::decode_max_output_size(enc_result.clone().unwrap().len())];
        let dec_result = cobsr::decode_array(&mut decode_out_vec[..], &enc_result.unwrap());
        assert!(dec_result.is_ok());
        assert_eq!(
            dec_result.unwrap(),
            mapping.rawdata,
            "{}",
            mapping.description
        );
    }
}

#[test]
fn test_cobs_decode_array_predefined() {
    for mapping in PREDEFINED_DECODINGS.iter() {
        let mut decode_out_vec = vec![0_u8; cobs::decode_max_output_size(mapping.encoded.len())];
        let dec_result = cobs::decode_array(&mut decode_out_vec[..], mapping.encoded);
        assert!(dec_result.is_ok());
        assert_eq!(
            dec_result.unwrap(),
            mapping.rawdata,
            "{}",
            mapping.description
        );

        // COBS/R decode function should also be able to decode COBS-encoded rawdata.
        let mut decode_out_vec = vec![0_u8; cobsr::decode_max_output_size(mapping.encoded.len())];
        let dec_result = cobsr::decode_array(&mut decode_out_vec[..], mapping.encoded);
        assert!(dec_result.is_ok());
        assert_eq!(
            dec_result.unwrap(),
            mapping.rawdata,
            "{}",
            mapping.description
        );
    }
}

#[test]
fn test_cobs_encode_array_buffer_too_small() {
    {
        let in_data = b"\x01\x01\x01\x01\x01";
        let mut cobs_encode_buf = [0xCC_u8; 5];
        let result = cobs::encode_array(&mut cobs_encode_buf, in_data);
        assert_eq!(result, Err(::cobs::Error::OutputBufferTooSmall));
    }

    {
        let in_data = b"\x01\x01\x01\x01\x01";
        let mut cobs_encode_buf = [0xCC_u8; 6];
        let result = cobs::encode_array(&mut cobs_encode_buf, in_data);
        assert_ne!(result, Err(::cobs::Error::OutputBufferTooSmall));
    }

    {
        let in_data = b"\x00\x00\x00\x00\x00";
        let mut cobs_encode_buf = [0xCC_u8; 5];
        let result = cobs::encode_array(&mut cobs_encode_buf, in_data);
        assert_eq!(result, Err(::cobs::Error::OutputBufferTooSmall));
    }

    {
        let in_data = b"\x00\x00\x00\x00\x00";
        let mut cobs_encode_buf = [0xCC_u8; 6];
        let result = cobs::encode_array(&mut cobs_encode_buf, in_data);
        assert_ne!(result, Err(::cobs::Error::OutputBufferTooSmall));
    }
}

#[test]
fn test_cobs_decode_array_buffer_too_small() {
    {
        let cobs_encoded_data = b"\x05AAAA";
        let mut cobs_decode_buf = [0xCC_u8; 3];
        let result = cobs::decode_array(&mut cobs_decode_buf, cobs_encoded_data);
        assert_eq!(result, Err(::cobs::Error::OutputBufferTooSmall));
    }

    {
        let cobs_encoded_data = b"\x05AAAA";
        let mut cobs_decode_buf = [0xCC_u8; 5];
        let result = cobs::decode_array(&mut cobs_decode_buf, cobs_encoded_data);
        assert_ne!(result, Err(::cobs::Error::OutputBufferTooSmall));
    }
}

#[test]
fn test_cobs_decode_array_bad() {
    // Try decoding bad data.
    let mut cobs_decode_buf = [0xCC_u8; 50];

    {
        let bad_cobs_encoded_data = b"\x00sAAA";
        let result = cobs::decode_array(&mut cobs_decode_buf, bad_cobs_encoded_data);
        assert_eq!(result, Err(::cobs::Error::ZeroInEncodedData));
    }

    {
        let bad_cobs_encoded_data = b"\x05AAA";
        let result = cobs::decode_array(&mut cobs_decode_buf, bad_cobs_encoded_data);
        assert_eq!(result, Err(::cobs::Error::TruncatedEncodedData));
    }

    {
        let bad_cobs_encoded_data = b"\x05\x00AAA";
        let result = cobs::decode_array(&mut cobs_decode_buf, bad_cobs_encoded_data);
        assert_eq!(result, Err(::cobs::Error::ZeroInEncodedData));
    }
}

#[cfg(feature = "alloc")]
#[test]
fn test_cobs_vector_predefined() {
    for mapping in PREDEFINED_ENCODINGS.iter() {
        let enc_result = cobs::encode_vector(mapping.rawdata);
        assert!(enc_result.is_ok());
        assert_eq!(
            enc_result.clone().unwrap(),
            mapping.encoded,
            "{}",
            mapping.description
        );

        let dec_result = cobs::decode_vector(&enc_result.clone().unwrap());
        assert!(dec_result.is_ok());
        assert_eq!(
            dec_result.unwrap(),
            mapping.rawdata,
            "{}",
            mapping.description
        );

        // COBS/R decode function should also be able to decode COBS-encoded rawdata.
        let dec_result = cobsr::decode_vector(&enc_result.unwrap());
        assert!(dec_result.is_ok());
        assert_eq!(
            dec_result.unwrap(),
            mapping.rawdata,
            "{}",
            mapping.description
        );
    }
}

#[cfg(feature = "alloc")]
#[test]
fn test_cobs_decode_vector_predefined() {
    for mapping in PREDEFINED_DECODINGS.iter() {
        let dec_result = cobs::decode_vector(mapping.encoded);
        assert!(dec_result.is_ok());
        assert_eq!(
            dec_result.unwrap(),
            mapping.rawdata,
            "{}",
            mapping.description
        );

        // COBS/R decode function should also be able to decode COBS-encoded rawdata.
        let dec_result = cobsr::decode_vector(mapping.encoded);
        assert!(dec_result.is_ok());
        assert_eq!(
            dec_result.unwrap(),
            mapping.rawdata,
            "{}",
            mapping.description
        );
    }
}

#[cfg(feature = "alloc")]
#[test]
fn test_cobs_stream_predefined() {
    for mapping in PREDEFINED_ENCODINGS.iter() {
        let encode_in_vec = mapping.rawdata.to_vec();
        let mut encode_out_vec = Vec::<u8>::new();
        let enc_result = cobs::encode_stream(&mut &encode_in_vec[..], &mut encode_out_vec);
        assert!(enc_result.is_ok());
        assert_eq!(
            encode_out_vec.clone(),
            mapping.encoded,
            "{}",
            mapping.description
        );
    }
}
