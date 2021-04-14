use ::cobs::cobsr;

struct DataEncodedMapping<'a> {
    pub rawdata: &'a [u8],
    pub encoded: &'a [u8],
}

const PREDEFINED_ENCODINGS: [DataEncodedMapping; 28] = [
    DataEncodedMapping{ rawdata: b"",                                   encoded: b"\x01"                            },
    DataEncodedMapping{ rawdata: b"\x01",                               encoded: b"\x02\x01"                        },
    DataEncodedMapping{ rawdata: b"\x02",                               encoded: b"\x02"                            },
    DataEncodedMapping{ rawdata: b"\x03",                               encoded: b"\x03"                            },
    DataEncodedMapping{ rawdata: b"\x7E",                               encoded: b"\x7E"                            },
    DataEncodedMapping{ rawdata: b"\x7F",                               encoded: b"\x7F"                            },
    DataEncodedMapping{ rawdata: b"\x80",                               encoded: b"\x80"                            },
    DataEncodedMapping{ rawdata: b"\xD5",                               encoded: b"\xD5"                            },
    DataEncodedMapping{ rawdata: b"\xFE",                               encoded: b"\xFE"                            },
    DataEncodedMapping{ rawdata: b"\xFF",                               encoded: b"\xFF"                            },
    DataEncodedMapping{ rawdata: b"a\x02",                              encoded: b"\x03a\x02"                       },
    DataEncodedMapping{ rawdata: b"a\x03",                              encoded: b"\x03a"                           },
    DataEncodedMapping{ rawdata: b"a\xFF",                              encoded: b"\xFFa"                           },
    DataEncodedMapping{ rawdata: b"\x05\x04\x03\x02\x01",               encoded: b"\x06\x05\x04\x03\x02\x01"        },
    DataEncodedMapping{ rawdata: b"12345",                              encoded: b"51234"                           },
    DataEncodedMapping{ rawdata: b"12345\x00\x04\x03\x02\x01",          encoded: b"\x0612345\x05\x04\x03\x02\x01"   },
    DataEncodedMapping{ rawdata: b"12345\x006789",                      encoded: b"\x06123459678"                   },
    DataEncodedMapping{ rawdata: b"\x0012345\x006789",                  encoded: b"\x01\x06123459678"               },
    DataEncodedMapping{ rawdata: b"12345\x006789\x00",                  encoded: b"\x0612345\x056789\x01"           },
    DataEncodedMapping{ rawdata: b"\x00",                               encoded: b"\x01\x01"                        },
    DataEncodedMapping{ rawdata: b"\x00\x00",                           encoded: b"\x01\x01\x01"                    },
    DataEncodedMapping{ rawdata: b"\x00\x00\x00",                       encoded: b"\x01\x01\x01\x01"                },
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
];

#[test]
fn test_cobsr_vector_predefined() {
    for mapping in PREDEFINED_ENCODINGS.iter() {
        let enc_result = cobsr::encode_vector(mapping.rawdata);
        assert!(enc_result.is_ok());
        assert_eq!(&enc_result.clone().unwrap(), mapping.encoded);

        let dec_result = cobsr::decode_vector(&enc_result.unwrap());
        assert!(dec_result.is_ok());
        assert_eq!(dec_result.unwrap(), mapping.rawdata);
    }
}