use ::cobs::{cobs, cobsr};

struct DataEncodedMapping<'a> {
    pub rawdata: &'a [u8],
    pub encoded: &'a [u8],
}

const PREDEFINED_ENCODINGS: [DataEncodedMapping; 16] = [
    DataEncodedMapping { rawdata: b"",                      encoded: b"\x01"                        },
    DataEncodedMapping { rawdata: b"1",                     encoded: b"\x021"                       },
    DataEncodedMapping { rawdata: b"12345",                 encoded: b"\x0612345"                   },
    DataEncodedMapping { rawdata: b"12345\x006789",         encoded: b"\x0612345\x056789"           },
    DataEncodedMapping { rawdata: b"\x0012345\x006789",     encoded: b"\x01\x0612345\x056789"       },
    DataEncodedMapping { rawdata: b"12345\x006789\x00",     encoded: b"\x0612345\x056789\x01"       },
    DataEncodedMapping { rawdata: b"\x00",                  encoded: b"\x01\x01"                    },
    DataEncodedMapping { rawdata: b"\x00\x00",              encoded: b"\x01\x01\x01"                },
    DataEncodedMapping { rawdata: b"\x00\x00\x00",          encoded: b"\x01\x01\x01\x01"            },
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
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234\x025",
    },
    DataEncodedMapping {
        // zero followed by 255 non-zero bytes
        rawdata: b"\x000123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst12345",
        encoded: b"\x01\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234\x025",
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
const PREDEFINED_DECODINGS: [DataEncodedMapping; 2] = [
    // Handle an empty string, returning an empty string.
    DataEncodedMapping { rawdata: b"",                      encoded: b""                            },
    DataEncodedMapping {
        // 254 non-zero bytes
        rawdata: b"0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234",
        // A naive encoder implementation might not handle this edge case optimally, and append a redundant trailing \x01.
        encoded: b"\xFF0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst0123456789ABCDEFGHIJKLMNOPQRSTabcdefghijklmnopqrst1234\x01",
    },
];

#[test]
fn test_cobs_array_predefined() {
    for mapping in PREDEFINED_ENCODINGS.iter() {
        let mut encode_out_vec = vec![0_u8; cobs::encode_max_output_size(mapping.rawdata.len())];
        let enc_result = cobs::encode_array(&mut encode_out_vec[..], mapping.rawdata);
        assert!(enc_result.is_ok());
        assert_eq!(enc_result.clone().unwrap(), mapping.encoded);

        let mut decode_out_vec = vec![0_u8; cobs::decode_max_output_size(enc_result.clone().unwrap().len())];
        let dec_result = cobs::decode_array(&mut decode_out_vec[..], &enc_result.clone().unwrap());
        assert!(dec_result.is_ok());
        assert_eq!(dec_result.unwrap(), mapping.rawdata);

        // COBS/R decode function should also be able to decode COBS-encoded rawdata.
        let mut decode_out_vec = vec![0_u8; cobsr::decode_max_output_size(enc_result.clone().unwrap().len())];
        let dec_result = cobsr::decode_array(&mut decode_out_vec[..], &enc_result.unwrap());
        assert!(dec_result.is_ok());
        assert_eq!(dec_result.unwrap(), mapping.rawdata);
    }
}

#[test]
fn test_cobs_decode_array_predefined() {
    for mapping in PREDEFINED_DECODINGS.iter() {
        let mut decode_out_vec = vec![0_u8; cobs::decode_max_output_size(mapping.encoded.len())];
        let dec_result = cobs::decode_array(&mut decode_out_vec[..],mapping.encoded);
        assert!(dec_result.is_ok());
        assert_eq!(dec_result.unwrap(), mapping.rawdata);

        // COBS/R decode function should also be able to decode COBS-encoded rawdata.
        let mut decode_out_vec = vec![0_u8; cobsr::decode_max_output_size(mapping.encoded.len())];
        let dec_result = cobsr::decode_array(&mut decode_out_vec[..], mapping.encoded);
        assert!(dec_result.is_ok());
        assert_eq!(dec_result.unwrap(), mapping.rawdata);
    }
}

#[test]
fn test_cobs_vector_predefined() {
    for mapping in PREDEFINED_ENCODINGS.iter() {
        let enc_result = cobs::encode_vector(mapping.rawdata);
        assert!(enc_result.is_ok());
        assert_eq!(enc_result.clone().unwrap(), mapping.encoded);

        let dec_result = cobs::decode_vector(&enc_result.clone().unwrap());
        assert!(dec_result.is_ok());
        assert_eq!(dec_result.unwrap(), mapping.rawdata);

        // COBS/R decode function should also be able to decode COBS-encoded rawdata.
        let dec_result = cobsr::decode_vector(&enc_result.unwrap());
        assert!(dec_result.is_ok());
        assert_eq!(dec_result.unwrap(), mapping.rawdata);
    }
}

#[test]
fn test_cobs_decode_vector_predefined() {
    for mapping in PREDEFINED_DECODINGS.iter() {
        let dec_result = cobs::decode_vector(mapping.encoded);
        assert!(dec_result.is_ok());
        assert_eq!(dec_result.unwrap(), mapping.rawdata);

        // COBS/R decode function should also be able to decode COBS-encoded rawdata.
        let dec_result = cobsr::decode_vector(mapping.encoded);
        assert!(dec_result.is_ok());
        assert_eq!(dec_result.unwrap(), mapping.rawdata);
    }
}
