use ::cobs::cobsr;

struct DataEncodedMapping<'a> {
    pub data: &'a [u8],
    pub encoded: &'a [u8],
}

const PREDEFINED_ENCODINGS: [DataEncodedMapping; 20] = [
    DataEncodedMapping{ data: b"",                                  encoded: b"\x01"                            },
    DataEncodedMapping{ data: b"\x01",                              encoded: b"\x02\x01"                        },
    DataEncodedMapping{ data: b"\x02",                              encoded: b"\x02"                            },
    DataEncodedMapping{ data: b"\x03",                              encoded: b"\x03"                            },
    DataEncodedMapping{ data: b"\x7E",                              encoded: b"\x7E"                            },
    DataEncodedMapping{ data: b"\x7F",                              encoded: b"\x7F"                            },
    DataEncodedMapping{ data: b"\x80",                              encoded: b"\x80"                            },
    DataEncodedMapping{ data: b"\xD5",                              encoded: b"\xD5"                            },
    DataEncodedMapping{ data: b"\xFE",                              encoded: b"\xFE"                            },
    DataEncodedMapping{ data: b"\xFF",                              encoded: b"\xFF"                            },
    DataEncodedMapping{ data: b"1",                                 encoded: b"1"                               },
    DataEncodedMapping{ data: b"\x05\x04\x03\x02\x01",              encoded: b"\x06\x05\x04\x03\x02\x01"        },
    DataEncodedMapping{ data: b"12345",                             encoded: b"51234"                           },
    DataEncodedMapping{ data: b"12345\x00\x04\x03\x02\x01",         encoded: b"\x0612345\x05\x04\x03\x02\x01"   },
    DataEncodedMapping{ data: b"12345\x006789",                     encoded: b"\x06123459678"                   },
    DataEncodedMapping{ data: b"\x0012345\x006789",                 encoded: b"\x01\x06123459678"               },
    DataEncodedMapping{ data: b"12345\x006789\x00",                 encoded: b"\x0612345\x056789\x01"           },
    DataEncodedMapping{ data: b"\x00",                              encoded: b"\x01\x01"                        },
    DataEncodedMapping{ data: b"\x00\x00",                          encoded: b"\x01\x01\x01"                    },
    DataEncodedMapping{ data: b"\x00\x00\x00",                      encoded: b"\x01\x01\x01\x01"                },
];

#[test]
fn test_cobsr_vector_predefined() {
    for mapping in PREDEFINED_ENCODINGS.iter() {
        let enc_result = cobsr::encode_vector(mapping.data);
        assert!(enc_result.is_ok());
        assert_eq!(&enc_result.clone().unwrap(), mapping.encoded);

        let dec_result = cobsr::decode_vector(&enc_result.unwrap());
        assert!(dec_result.is_ok());
        assert_eq!(dec_result.unwrap(), mapping.data);
    }
}