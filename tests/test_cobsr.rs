use ::cobs::cobsr;

struct DataEncodedMapping<'a> {
    pub rawdata: &'a [u8],
    pub encoded: &'a [u8],
}

const PREDEFINED_ENCODINGS: [DataEncodedMapping; 31] = [
    DataEncodedMapping { rawdata: b"",                                  encoded: b"\x01"                            },
    DataEncodedMapping { rawdata: b"\x01",                              encoded: b"\x02\x01"                        },
    DataEncodedMapping { rawdata: b"\x02",                              encoded: b"\x02"                            },
    DataEncodedMapping { rawdata: b"\x03",                              encoded: b"\x03"                            },
    DataEncodedMapping { rawdata: b"\x7E",                              encoded: b"\x7E"                            },
    DataEncodedMapping { rawdata: b"\x7F",                              encoded: b"\x7F"                            },
    DataEncodedMapping { rawdata: b"\x80",                              encoded: b"\x80"                            },
    DataEncodedMapping { rawdata: b"\xD5",                              encoded: b"\xD5"                            },
    DataEncodedMapping { rawdata: b"\xFE",                              encoded: b"\xFE"                            },
    DataEncodedMapping { rawdata: b"\xFF",                              encoded: b"\xFF"                            },
    DataEncodedMapping { rawdata: b"a\x02",                             encoded: b"\x03a\x02"                       },
    DataEncodedMapping { rawdata: b"a\x03",                             encoded: b"\x03a"                           },
    DataEncodedMapping { rawdata: b"a\xFF",                             encoded: b"\xFFa"                           },
    DataEncodedMapping { rawdata: b"\x05\x04\x03\x02\x01",              encoded: b"\x06\x05\x04\x03\x02\x01"        },
    DataEncodedMapping { rawdata: b"12345",                             encoded: b"51234"                           },
    DataEncodedMapping { rawdata: b"12345\x00\x04\x03\x02\x01",         encoded: b"\x0612345\x05\x04\x03\x02\x01"   },
    DataEncodedMapping { rawdata: b"12345\x006789",                     encoded: b"\x06123459678"                   },
    DataEncodedMapping { rawdata: b"\x0012345\x006789",                 encoded: b"\x01\x06123459678"               },
    DataEncodedMapping { rawdata: b"12345\x006789\x00",                 encoded: b"\x0612345\x056789\x01"           },
    DataEncodedMapping { rawdata: b"\x00",                              encoded: b"\x01\x01"                        },
    DataEncodedMapping { rawdata: b"\x00\x00",                          encoded: b"\x01\x01\x01"                    },
    DataEncodedMapping { rawdata: b"\x00\x00\x00",                      encoded: b"\x01\x01\x01\x01"                },
    DataEncodedMapping {
        // 253 non-zero bytes
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123",
        encoded: b"\xFE0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123",
    },
    DataEncodedMapping {
        // 254 non-zero bytes
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234",
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234",
    },
    DataEncodedMapping {
        // 255 non-zero bytes
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst12345",
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst12345",
    },
    DataEncodedMapping {
        // zero followed by 255 non-zero bytes
        rawdata: b"\x000123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst12345",
        encoded: b"\x01\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst12345",
    },
    DataEncodedMapping {
        // 254 non-zero bytes, ending with a final FE
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123\xFE",
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123\xFE",
    },
    DataEncodedMapping {
        // 254 non-zero bytes, ending with a final FF
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123\xFF",
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123",
    },
    DataEncodedMapping {
        // 253 non-zero bytes followed by zero
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123\x00",
        encoded: b"\xFE0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123\x01",
    },
    DataEncodedMapping {
        // 254 non-zero bytes followed by zero
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234\x00",
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234\x01\x01",
    },
    DataEncodedMapping {
        // 255 non-zero bytes followed by zero
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst12345\x00",
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234\x025\x01",
    },
];

/*
 * Decoding-specific tests. These are for unusual encoded data, which a correct encoder wouldn't normally generate, but
 * could be encountered from a different encoder implementation that generates non-optimal encodings.
 */
const PREDEFINED_DECODINGS: [DataEncodedMapping; 3] = [
    // Handle an empty string, returning an empty string.
    DataEncodedMapping { rawdata: b"",                      encoded: b""                            },
    DataEncodedMapping {
        // 254 non-zero bytes
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234",
        // A naive encoder implementation might not handle this edge case optimally, and append a redundant trailing \x01.
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234\x01",
    },
    DataEncodedMapping {
        // 254 non-zero bytes, ending with a final FF
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123\xFF",
        // A naive COBS/R encoder implementation might not handle this edge case optimally, and output a trailing \xFF.
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst123\xFF",
    },
];

#[test]
fn test_cobsr_array_predefined() {
    for mapping in PREDEFINED_ENCODINGS.iter() {
        let mut encode_out_vec = vec![0_u8; cobsr::encode_max_output_size(mapping.rawdata.len())];
        let enc_result = cobsr::encode_array(&mut encode_out_vec[..], mapping.rawdata);
        assert!(enc_result.is_ok());
        assert_eq!(enc_result.clone().unwrap(), mapping.encoded);

        let mut decode_out_vec = vec![0_u8; cobsr::decode_max_output_size(enc_result.clone().unwrap().len())];
        let dec_result = cobsr::decode_array(&mut decode_out_vec[..], &enc_result.unwrap());
        assert!(dec_result.is_ok());
        assert_eq!(dec_result.unwrap(), mapping.rawdata);
    }
}

#[test]
fn test_cobsr_decode_array_predefined() {
    for mapping in PREDEFINED_DECODINGS.iter() {
        let mut decode_out_vec = vec![0_u8; cobsr::decode_max_output_size(mapping.encoded.len())];
        let dec_result = cobsr::decode_array(&mut decode_out_vec[..],mapping.encoded);
        assert!(dec_result.is_ok());
        assert_eq!(dec_result.unwrap(), mapping.rawdata);
    }
}

#[test]
fn test_cobsr_vector_predefined() {
    for mapping in PREDEFINED_ENCODINGS.iter() {
        let enc_result = cobsr::encode_vector(mapping.rawdata);
        assert!(enc_result.is_ok());
        assert_eq!(enc_result.clone().unwrap(), mapping.encoded);

        let dec_result = cobsr::decode_vector(&enc_result.unwrap());
        assert!(dec_result.is_ok());
        assert_eq!(dec_result.unwrap(), mapping.rawdata);
    }
}

#[test]
fn test_cobsr_decode_vector_predefined() {
    for mapping in PREDEFINED_DECODINGS.iter() {
        let dec_result = cobsr::decode_vector(mapping.encoded);
        assert!(dec_result.is_ok());
        assert_eq!(dec_result.unwrap(), mapping.rawdata);
    }
}
