use hex;

mod cobs {
    pub fn encode<'a>(out_buf: &'a mut [u8], in_buf: &[u8]) -> isize {
        //out_buf[0] = 1u8;
        //1
        let mut code_i = 0;
        let mut out_i = 1;

        for x in in_buf {
            if *x == 0 {
                out_buf[code_i] = (out_i - code_i) as u8;
                code_i = out_i;
                out_i = code_i + 1;
            }
            else {
                out_buf[out_i] = *x;
                out_i += 1;
                if out_i - code_i >= 0xFE {
                    out_buf[code_i] = 0xFF;
                    code_i = out_i;
                    out_i = code_i + 1;
                }
            }
        }
        
        /* We've reached the end of the source data.
         * Finalise the remaining output. In particular, write the code (length) byte.
         * Update the pointer to calculate the final output length.
         */
        out_buf[code_i] = (out_i - code_i) as u8;
        
        out_i as isize
    }
}

fn main() {
    let data = b"123\x00456";
    let mut data_cobs: [u8; 9] = [ 0; 9 ];
    
    let encode_len = cobs::encode(&mut data_cobs, data);
    if encode_len < 0 {
        println!("COBS encode error");
    }
    else {
        println!("COBS encode: {} length {}", hex::encode(data_cobs), encode_len);
    }
}
