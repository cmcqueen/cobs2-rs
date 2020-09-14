
#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub enum Error {
    OutputBufferTooSmall,
    ZeroInEncodedData,
    TruncatedEncodedData,
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::OutputBufferTooSmall => {
                write!(f, "Output buffer is too small")
            }
            Error::ZeroInEncodedData => {
                write!(f, "Zero found in encoded input data")
            }
            Error::TruncatedEncodedData => {
                write!(f, "Unexpected end of encoded input data")
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, crate::Error>;

pub mod cobs {
    pub const fn encode_max_output_size(input_len: usize) -> usize {
        input_len + ((input_len + 253) / 254)
    }

    pub const fn decode_max_output_size(input_len: usize) -> usize {
        if input_len > 0 {
            input_len - 1
        } else {
            0
        }
    }

    pub fn encode<'a>(out_buf: &'a mut [u8], in_buf: &[u8]) -> crate::Result<&'a[u8]> {
        let mut code_i = 0;
        let mut out_i = 1;

        for x in in_buf {
            if *x == 0 {
                if code_i >= out_buf.len() {
                    return Err(crate::Error::OutputBufferTooSmall);
                }
                out_buf[code_i] = (out_i - code_i) as u8;
                code_i = out_i;
                out_i = code_i + 1;
            }
            else {
                if out_i >= out_buf.len() {
                    return Err(crate::Error::OutputBufferTooSmall);
                }
                out_buf[out_i] = *x;
                out_i += 1;
                if out_i - code_i >= 0xFE {
                    if code_i >= out_buf.len() {
                        return Err(crate::Error::OutputBufferTooSmall);
                    }
                    out_buf[code_i] = 0xFF;
                    code_i = out_i;
                    out_i = code_i + 1;
                }
            }
        }

        // We've reached the end of the source data.
        // Finalise the remaining output. In particular, write the code (length) byte.
        // Update the pointer to calculate the final output length.
        if code_i >= out_buf.len() {
            return Err(crate::Error::OutputBufferTooSmall);
        }
        out_buf[code_i] = (out_i - code_i) as u8;

        Ok(&out_buf[..out_i])
    }

    pub fn decode<'a>(out_buf: &'a mut [u8], in_buf: &[u8]) -> crate::Result<&'a[u8]> {
        let mut code_i = 0;
        let mut out_i = 0;

        if in_buf.len() != 0 {
            loop {
                let code = in_buf[code_i];
                if code == 0 {
                    return Err(crate::Error::ZeroInEncodedData);
                }
                for in_i in (code_i + 1)..(code_i + code as usize) {
                    if in_i >= in_buf.len() {
                        return Err(crate::Error::TruncatedEncodedData);
                    }
                    let in_byte = in_buf[in_i];
                    if in_byte == 0 {
                        return Err(crate::Error::ZeroInEncodedData);
                    }
                    if out_i >= out_buf.len() {
                        return Err(crate::Error::OutputBufferTooSmall);
                    }
                    out_buf[out_i] = in_byte;
                    out_i += 1;
                }
                code_i += code as usize;
                if code_i >= in_buf.len() {
                    // End of data. Exit, without outputting a trailing zero for the end of the data.
                    break;
                }
                if code < 0xFF {
                    // Output trailing zero.
                    if out_i >= out_buf.len() {
                        return Err(crate::Error::OutputBufferTooSmall);
                    }
                    out_buf[out_i] = 0;
                    out_i += 1;
                }
            }
        }
        Ok(&out_buf[..out_i])
    }
}

pub mod cobsr {
    pub const fn encode_max_output_size(input_len: usize) -> usize {
        input_len + ((input_len + 253) / 254)
    }

    pub const fn decode_max_output_size(input_len: usize) -> usize {
        input_len
    }

    pub fn encode<'a>(out_buf: &'a mut [u8], in_buf: &[u8]) -> crate::Result<&'a[u8]> {
        let mut code_i = 0;
        let mut out_i = 1;
        let mut last_value = 0u8;

        for x in in_buf {
            if *x == 0 {
                if code_i >= out_buf.len() {
                    return Err(crate::Error::OutputBufferTooSmall);
                }
                out_buf[code_i] = (out_i - code_i) as u8;
                code_i = out_i;
                out_i = code_i + 1;
                last_value = 0u8;
            }
            else {
                last_value = *x;
                if out_i >= out_buf.len() {
                    return Err(crate::Error::OutputBufferTooSmall);
                }
                out_buf[out_i] = last_value;
                out_i += 1;
                if out_i - code_i >= 0xFE {
                    if code_i >= out_buf.len() {
                        return Err(crate::Error::OutputBufferTooSmall);
                    }
                    out_buf[code_i] = 0xFF;
                    code_i = out_i;
                    out_i = code_i + 1;
                }
            }
        }

        // We've reached the end of the source data.
        // Finalise the remaining output. In particular, write the code (length) byte.
        // Update the pointer to calculate the final output length.
        if code_i >= out_buf.len() {
            return Err(crate::Error::OutputBufferTooSmall);
        }
        if last_value >= (out_i - code_i) as u8 {
            out_buf[code_i] = last_value;
            out_i -= 1;
        }
        else {
            out_buf[code_i] = (out_i - code_i) as u8;
        }

        Ok(&out_buf[..out_i])
    }

    pub fn decode<'a>(out_buf: &'a mut [u8], in_buf: &[u8]) -> crate::Result<&'a[u8]> {
        let mut code_i = 0;
        let mut out_i = 0;

        if in_buf.len() != 0 {
            loop {
                let code = in_buf[code_i];
                if code == 0 {
                    return Err(crate::Error::ZeroInEncodedData);
                }
                for in_i in (code_i + 1)..(code_i + code as usize) {
                    if in_i >= in_buf.len() {
                        // End of data, where length code is greater than remaining data.
                        // Output the length code as the last output byte.
                        if out_i >= out_buf.len() {
                            return Err(crate::Error::OutputBufferTooSmall);
                        }
                        out_buf[out_i] = code;
                        out_i += 1;
                        break;
                    }
                    let in_byte = in_buf[in_i];
                    if in_byte == 0 {
                        return Err(crate::Error::ZeroInEncodedData);
                    }
                    if out_i >= out_buf.len() {
                        return Err(crate::Error::OutputBufferTooSmall);
                    }
                    out_buf[out_i] = in_byte;
                    out_i += 1;
                }
                code_i += code as usize;
                if code_i >= in_buf.len() {
                    // End of data. Exit, without outputting a trailing zero for the end of the data.
                    break;
                }
                if code < 0xFF {
                    // Output trailing zero.
                    if out_i >= out_buf.len() {
                        return Err(crate::Error::OutputBufferTooSmall);
                    }
                    out_buf[out_i] = 0;
                    out_i += 1;
                }
            }
        }
        Ok(&out_buf[..out_i])
    }
}
