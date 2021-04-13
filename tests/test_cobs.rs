use ::cobs::cobs;

struct DataEncodedMapping<'a> {
    pub data: &'a [u8],
    pub encoded: &'a [u8],
}

const PREDEFINED_ENCODINGS: [DataEncodedMapping; 9] = [
    DataEncodedMapping{ data: b"",                      encoded: b"\x01"                        },
    DataEncodedMapping{ data: b"1",                     encoded: b"\x021"                       },
    DataEncodedMapping{ data: b"12345",                 encoded: b"\x0612345"                   },
    DataEncodedMapping{ data: b"12345\x006789",         encoded: b"\x0612345\x056789"           },
    DataEncodedMapping{ data: b"\x0012345\x006789",     encoded: b"\x01\x0612345\x056789"       },
    DataEncodedMapping{ data: b"12345\x006789\x00",     encoded: b"\x0612345\x056789\x01"       },
    DataEncodedMapping{ data: b"\x00",                  encoded: b"\x01\x01"                    },
    DataEncodedMapping{ data: b"\x00\x00",              encoded: b"\x01\x01\x01"                },
    DataEncodedMapping{ data: b"\x00\x00\x00",          encoded: b"\x01\x01\x01\x01"            },
];

#[test]
fn test_cobs_vector_predefined() {
    for mapping in PREDEFINED_ENCODINGS.iter() {
        let enc_result = cobs::encode_vector(mapping.data);
        assert!(enc_result.is_ok());
        assert_eq!(&enc_result.clone().unwrap(), mapping.encoded);

        let dec_result = cobs::decode_vector(&enc_result.unwrap());
        assert!(dec_result.is_ok());
        assert_eq!(dec_result.unwrap(), mapping.data);
    }
}