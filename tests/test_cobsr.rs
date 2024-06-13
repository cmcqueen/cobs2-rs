use ::cobs::cobsr;

struct DataEncodedMapping<'a> {
    pub description: &'a str,
    pub rawdata: &'a [u8],
    pub encoded: &'a [u8],
}

const PREDEFINED_ENCODINGS: [DataEncodedMapping; 31] = [
    DataEncodedMapping { description: "empty",                          rawdata: b"",                                  encoded: b"\x01"                            },
    DataEncodedMapping { description: "1 byte 0x01",                    rawdata: b"\x01",                              encoded: b"\x02\x01"                        },
    DataEncodedMapping { description: "1 byte 0x02",                    rawdata: b"\x02",                              encoded: b"\x02"                            },
    DataEncodedMapping { description: "1 byte 0x03",                    rawdata: b"\x03",                              encoded: b"\x03"                            },
    DataEncodedMapping { description: "1 byte 0x7E",                    rawdata: b"\x7E",                              encoded: b"\x7E"                            },
    DataEncodedMapping { description: "1 byte 0x7F",                    rawdata: b"\x7F",                              encoded: b"\x7F"                            },
    DataEncodedMapping { description: "1 byte 0x80",                    rawdata: b"\x80",                              encoded: b"\x80"                            },
    DataEncodedMapping { description: "1 byte 0xD5",                    rawdata: b"\xD5",                              encoded: b"\xD5"                            },
    DataEncodedMapping { description: "1 byte 0xFE",                    rawdata: b"\xFE",                              encoded: b"\xFE"                            },
    DataEncodedMapping { description: "1 byte 0xFF",                    rawdata: b"\xFF",                              encoded: b"\xFF"                            },
    DataEncodedMapping { description: "2 bytes ending 0x02",            rawdata: b"a\x02",                             encoded: b"\x03a\x02"                       },
    DataEncodedMapping { description: "2 bytes ending 0x03",            rawdata: b"a\x03",                             encoded: b"\x03a"                           },
    DataEncodedMapping { description: "2 bytes ending 0xFF",            rawdata: b"a\xFF",                             encoded: b"\xFFa"                           },
    DataEncodedMapping { description: "5 non-zero bytes ending 0x01",   rawdata: b"\x05\x04\x03\x02\x01",              encoded: b"\x06\x05\x04\x03\x02\x01"        },
    DataEncodedMapping { description: "5 non-zero bytes ending 0x35",   rawdata: b"12345",                             encoded: b"51234"                           },
    DataEncodedMapping { description: "zero in middle, ending 0x01",    rawdata: b"12345\x00\x04\x03\x02\x01",         encoded: b"\x0612345\x05\x04\x03\x02\x01"   },
    DataEncodedMapping { description: "zero in middle, ending 0x39",    rawdata: b"12345\x006789",                     encoded: b"\x06123459678"                   },
    DataEncodedMapping { description: "2 chunks starting with zero",    rawdata: b"\x0012345\x006789",                 encoded: b"\x01\x06123459678"               },
    DataEncodedMapping { description: "2 chunks ending with zero",      rawdata: b"12345\x006789\x00",                 encoded: b"\x0612345\x056789\x01"           },
    DataEncodedMapping { description: "1 zero",                         rawdata: b"\x00",                              encoded: b"\x01\x01"                        },
    DataEncodedMapping { description: "2 zeros",                        rawdata: b"\x00\x00",                          encoded: b"\x01\x01\x01"                    },
    DataEncodedMapping { description: "3 zeros",                        rawdata: b"\x00\x00\x00",                      encoded: b"\x01\x01\x01\x01"                },
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
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst12345",
    },
    DataEncodedMapping {
        description: "zero followed by 255 non-zero bytes",
        rawdata: b"\x000123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst12345",
        encoded: b"\x01\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst12345",
    },
    DataEncodedMapping {
        description: "254 non-zero bytes, ending with a final FE",
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123\xFE",
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123\xFE",
    },
    DataEncodedMapping {
        description: "254 non-zero bytes, ending with a final FF",
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123\xFF",
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123",
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
const PREDEFINED_DECODINGS: [DataEncodedMapping; 11] = [
    // Handle an empty string, returning an empty string.
    DataEncodedMapping { description: "empty",                          rawdata: b"",                                  encoded: b""                                },
    DataEncodedMapping { description: "1 byte 0x02",                    rawdata: b"\x02",                              encoded: b"\x02\x02"                        },
    DataEncodedMapping { description: "1 byte 0x03",                    rawdata: b"\x03",                              encoded: b"\x02\x03"                        },
    DataEncodedMapping { description: "1 byte 0xFE",                    rawdata: b"\xFE",                              encoded: b"\x02\xFE"                        },
    DataEncodedMapping { description: "1 byte 0xFF",                    rawdata: b"\xFF",                              encoded: b"\x02\xFF"                        },
    DataEncodedMapping { description: "2 bytes ending 0x03",            rawdata: b"a\x03",                             encoded: b"\x03a\x03"                       },
    DataEncodedMapping { description: "2 bytes ending 0xFF",            rawdata: b"a\xFF",                             encoded: b"\x03a\xFF"                       },
    DataEncodedMapping { description: "5 non-zero bytes ending 0x35",   rawdata: b"12345",                             encoded: b"\x0612345"                       },
    DataEncodedMapping {
        description: "254 non-zero bytes",
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234",
        // A naive encoder implementation might not handle this edge case optimally, and append a redundant trailing \x01.
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234\x01",
    },
    DataEncodedMapping {
        description: "254 non-zero bytes, ending with a final FF, naive 1",
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123\xFF",
        // A naive COBS/R encoder implementation might not handle this edge case optimally, and output a trailing \xFF.
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123\xFF",
    },
    DataEncodedMapping {
        description: "254 non-zero bytes, ending with a final FF, naive 2",
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123\xFF",
        // A naive COBS/R encoder implementation might not handle this edge case optimally, and output a trailing \xFF AND a trailing \x01.
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123\xFF\x01",
    },
];

#[test]
fn test_cobsr_encode_max_output_size() {
    assert_eq!(1, cobsr::encode_max_output_size(0));
    assert_eq!(2, cobsr::encode_max_output_size(1));
    assert_eq!(3, cobsr::encode_max_output_size(2));

    assert_eq!(254, cobsr::encode_max_output_size(253));
    assert_eq!(255, cobsr::encode_max_output_size(254));
    assert_eq!(257, cobsr::encode_max_output_size(255));
    assert_eq!(258, cobsr::encode_max_output_size(256));

    assert_eq!(509, cobsr::encode_max_output_size(507));
    assert_eq!(510, cobsr::encode_max_output_size(508));
    assert_eq!(512, cobsr::encode_max_output_size(509));
    assert_eq!(513, cobsr::encode_max_output_size(510));

    assert_eq!(
        usize::max_value(),
        cobsr::encode_max_output_size(usize::max_value())
    );
    let increase = usize::max_value() / 255;
    assert_eq!(
        usize::max_value(),
        cobsr::encode_max_output_size(usize::max_value() - increase)
    );
}

#[test]
fn test_cobsr_array_predefined() {
    for mapping in PREDEFINED_ENCODINGS.iter() {
        let mut encode_out_vec = vec![0_u8; cobsr::encode_max_output_size(mapping.rawdata.len())];
        let enc_result = cobsr::encode_array(&mut encode_out_vec[..], mapping.rawdata);
        assert!(enc_result.is_ok());
        assert_eq!(
            enc_result.clone().unwrap(),
            mapping.encoded,
            "{}",
            mapping.description
        );

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
fn test_cobsr_decode_array_predefined() {
    for mapping in PREDEFINED_DECODINGS.iter() {
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
fn test_cobsr_encode_array_buffer_too_small() {
    {
        let in_data = b"\x01\x01\x01\x01\x01";
        let mut cobs_encode_buf = [0xCC_u8; 5];
        let result = cobsr::encode_array(&mut cobs_encode_buf, in_data);
        assert_eq!(result, Err(::cobs::Error::OutputBufferTooSmall));
    }

    {
        let in_data = b"\x01\x01\x01\x01\x01";
        let mut cobs_encode_buf = [0xCC_u8; 6];
        let result = cobsr::encode_array(&mut cobs_encode_buf, in_data);
        assert_ne!(result, Err(::cobs::Error::OutputBufferTooSmall));
    }

    {
        let in_data = b"\x00\x00\x00\x00\x00";
        let mut cobs_encode_buf = [0xCC_u8; 5];
        let result = cobsr::encode_array(&mut cobs_encode_buf, in_data);
        assert_eq!(result, Err(::cobs::Error::OutputBufferTooSmall));
    }

    {
        let in_data = b"\x00\x00\x00\x00\x00";
        let mut cobs_encode_buf = [0xCC_u8; 6];
        let result = cobsr::encode_array(&mut cobs_encode_buf, in_data);
        assert_ne!(result, Err(::cobs::Error::OutputBufferTooSmall));
    }
}

#[test]
fn test_cobsr_decode_array_buffer_too_small() {
    {
        let cobsr_encoded_data = b"\x05AAAA";
        let mut cobsr_decode_buf = [0xCC_u8; 3];
        let result = cobsr::decode_array(&mut cobsr_decode_buf, cobsr_encoded_data);
        assert_eq!(result, Err(::cobs::Error::OutputBufferTooSmall));
    }

    {
        let cobsr_encoded_data = b"\x05AAAA";
        let mut cobsr_decode_buf = [0xCC_u8; 5];
        let result = cobsr::decode_array(&mut cobsr_decode_buf, cobsr_encoded_data);
        assert_ne!(result, Err(::cobs::Error::OutputBufferTooSmall));
    }
}

#[test]
fn test_cobsr_decode_array_bad() {
    // Try decoding bad data.
    let mut cobsr_decode_buf = [0xCC_u8; 50];

    {
        let bad_cobsr_encoded_data = b"\x00sAAA";
        let result = cobsr::decode_array(&mut cobsr_decode_buf, bad_cobsr_encoded_data);
        assert_eq!(result, Err(::cobs::Error::ZeroInEncodedData));
    }

    {
        let bad_cobsr_encoded_data = b"\x05\x00AAA";
        let result = cobsr::decode_array(&mut cobsr_decode_buf, bad_cobsr_encoded_data);
        assert_eq!(result, Err(::cobs::Error::ZeroInEncodedData));
    }
}

#[cfg(feature = "alloc")]
#[test]
fn test_cobsr_vector_predefined() {
    for mapping in PREDEFINED_ENCODINGS.iter() {
        let enc_result = cobsr::encode_vector(mapping.rawdata);
        assert!(enc_result.is_ok());
        assert_eq!(
            enc_result.clone().unwrap(),
            mapping.encoded,
            "{}",
            mapping.description
        );

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
fn test_cobsr_decode_vector_predefined() {
    for mapping in PREDEFINED_DECODINGS.iter() {
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
fn test_cobsr_stream_predefined() {
    for mapping in PREDEFINED_ENCODINGS.iter() {
        let encode_in_vec = mapping.rawdata.to_vec();
        let mut encode_out_vec = Vec::<u8>::new();
        let enc_result = cobsr::encode_stream(&mut &encode_in_vec[..], &mut encode_out_vec);
        assert!(enc_result.is_ok());
        assert_eq!(
            encode_out_vec.clone(),
            mapping.encoded,
            "{}",
            mapping.description
        );
    }
}
