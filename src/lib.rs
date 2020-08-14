
#![allow(dead_code)]

pub mod cobs {
    pub fn encode<'a>(out_buf: &'a mut [u8], in_buf: &[u8]) -> &'a[u8] {
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

        &out_buf[..out_i]
    }

    pub fn decode<'a>(out_buf: &'a mut [u8], in_buf: &[u8]) -> &'a[u8] {
        let mut code_i = 0;
        let mut out_i = 0;

        if in_buf.len() != 0 {
            loop {
                let code = in_buf[code_i];
                for in_i in (code_i + 1)..(code_i + code as usize) {
                    if in_i >= in_buf.len() {
                        break;
                    }
                    out_buf[out_i] = in_buf[in_i];
                    out_i += 1;
                }
                code_i += code as usize;
                if code_i >= in_buf.len() {
                    break;
                }
                out_buf[out_i] = 0;
                out_i += 1;
            }
        }
        &out_buf[..out_i]
    }
}

pub mod cobsr {
    pub fn encode<'a>(out_buf: &'a mut [u8], in_buf: &[u8]) -> &'a[u8] {
        let mut code_i = 0;
        let mut out_i = 1;
        let mut last_value = 0u8;

        for x in in_buf {
            if *x == 0 {
                out_buf[code_i] = (out_i - code_i) as u8;
                code_i = out_i;
                out_i = code_i + 1;
                last_value = 0u8;
            }
            else {
                last_value = *x;
                out_buf[out_i] = last_value;
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
        if last_value >= (out_i - code_i) as u8 {
            out_buf[code_i] = last_value;
            out_i -= 1;
        }
        else {
            out_buf[code_i] = (out_i - code_i) as u8;
        }

        &out_buf[..out_i]
    }

    pub fn decode<'a>(out_buf: &'a mut [u8], in_buf: &[u8]) -> &'a[u8] {
        let mut code_i = 0;
        let mut out_i = 0;

        if in_buf.len() != 0 {
            loop {
                let code = in_buf[code_i];
                for in_i in (code_i + 1)..(code_i + code as usize) {
                    if in_i >= in_buf.len() {
                        out_buf[out_i] = code;
                        out_i += 1;
                        break;
                    }
                    out_buf[out_i] = in_buf[in_i];
                    out_i += 1;
                }
                code_i += code as usize;
                if code_i >= in_buf.len() {
                    break;
                }
                out_buf[out_i] = 0;
                out_i += 1;
            }
        }
        &out_buf[..out_i]
    }
}
