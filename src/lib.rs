
#![allow(dead_code)]

#[derive(Debug, PartialEq, Clone)]
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
        if input_len > 0 {
            input_len + ((input_len + 253) / 254)
        } else {
            1
        }
    }

    pub const fn decode_max_output_size(input_len: usize) -> usize {
        if input_len > 1 {
            input_len - 1
        } else {
            1
        }
    }

    pub fn encode_array<'a>(out_buf: &'a mut [u8], in_buf: &[u8]) -> crate::Result<&'a[u8]> {
        let mut code_i = 0;
        let mut out_i = 1;

        if code_i >= out_buf.len() {
            return Err(crate::Error::OutputBufferTooSmall);
        }
        for x in in_buf {
            if *x == 0 {
                out_buf[code_i] = (out_i - code_i) as u8;
                code_i = out_i;
                if code_i >= out_buf.len() {
                    return Err(crate::Error::OutputBufferTooSmall);
                }
                out_i = code_i + 1;
            }
            else {
                if out_i >= out_buf.len() {
                    return Err(crate::Error::OutputBufferTooSmall);
                }
                out_buf[out_i] = *x;
                out_i += 1;
                if out_i - code_i >= 0xFF {
                    out_buf[code_i] = 0xFF;
                    code_i = out_i;
                    if code_i >= out_buf.len() {
                        return Err(crate::Error::OutputBufferTooSmall);
                    }
                    out_i = code_i + 1;
                }
            }
        }

        // We've reached the end of the source data.
        // Finalise the remaining output. In particular, write the code (length) byte.
        // Update the pointer to calculate the final output length.
        out_buf[code_i] = (out_i - code_i) as u8;

        Ok(&out_buf[..out_i])
    }

    pub fn encode_vector(in_buf: &[u8]) -> crate::Result<std::vec::Vec<u8>> {
        let mut code_i = 0;
        let mut run_len = 0_u8;
        let mut out_vec = std::vec::Vec::<u8>::with_capacity(encode_max_output_size(in_buf.len()));

        for x in in_buf {
            if run_len >= 0xFF {
                out_vec[code_i] = 0xFF;
                code_i = out_vec.len();
                run_len = 0;
            }
            if *x == 0 {
                if run_len == 0 {
                    out_vec.push(1);
                }
                else {
                    out_vec[code_i] = run_len;
                }
                code_i = out_vec.len();
                run_len = 0;
            }
            else {
                if run_len == 0 {
                    out_vec.push(0xFF);
                    run_len = 1;
                }
                out_vec.push(*x);
                run_len += 1;
            }
        }

        // We've reached the end of the source data.
        // Finalise the remaining output. In particular, write the code (length) byte.
        if run_len == 0 {
            out_vec.push(1);
        }
        else {
            out_vec[code_i] = run_len;
        }

        Ok(out_vec)
    }

    pub fn decode_array<'a>(out_buf: &'a mut [u8], in_buf: &[u8]) -> crate::Result<&'a[u8]> {
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

    pub fn decode_vector(in_buf: &[u8]) -> crate::Result<std::vec::Vec<u8>> {
        let mut code_i = 0;
        let mut out_vec = std::vec::Vec::<u8>::with_capacity(decode_max_output_size(in_buf.len()));

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
                    out_vec.push(in_byte);
                }
                code_i += code as usize;
                if code_i >= in_buf.len() {
                    // End of data. Exit, without outputting a trailing zero for the end of the data.
                    break;
                }
                if code < 0xFF {
                    // Output trailing zero.
                    out_vec.push(0);
                }
            }
        }
        Ok(out_vec)
    }
}

pub mod cobsr {
    pub const fn encode_max_output_size(input_len: usize) -> usize {
        if input_len > 0 {
            input_len + ((input_len + 253) / 254)
        } else {
            1
        }
    }

    pub const fn decode_max_output_size(input_len: usize) -> usize {
        if input_len > 0 {
            input_len
        }
        else {
            1
        }
    }

    pub fn encode_array<'a>(out_buf: &'a mut [u8], in_buf: &[u8]) -> crate::Result<&'a[u8]> {
        let mut code_i = 0;
        let mut out_i = 1;
        let mut last_value = 0_u8;

        if code_i >= out_buf.len() {
            return Err(crate::Error::OutputBufferTooSmall);
        }
        for x in in_buf {
            if out_i - code_i >= 0xFF {
                out_buf[code_i] = 0xFF;
                code_i = out_i;
                if code_i >= out_buf.len() {
                    return Err(crate::Error::OutputBufferTooSmall);
                }
                out_i = code_i + 1;
            }
            if *x == 0 {
                out_buf[code_i] = (out_i - code_i) as u8;
                code_i = out_i;
                if code_i >= out_buf.len() {
                    return Err(crate::Error::OutputBufferTooSmall);
                }
                out_i = code_i + 1;
                last_value = 0;
            }
            else {
                last_value = *x;
                if out_i >= out_buf.len() {
                    return Err(crate::Error::OutputBufferTooSmall);
                }
                out_buf[out_i] = last_value;
                out_i += 1;
            }
        }

        // We've reached the end of the source data.
        // Finalise the remaining output. In particular, write the code (length) byte.
        // Update the pointer to calculate the final output length.
        if last_value >= (out_i - code_i) as u8 {
            out_buf[code_i] = last_value;
            out_i -= 1;
        }
        else {
            out_buf[code_i] = (out_i - code_i) as u8;
        }

        Ok(&out_buf[..out_i])
    }

    pub fn encode_vector(in_buf: &[u8]) -> crate::Result<std::vec::Vec<u8>> {
        let mut code_i = 0;
        let mut run_len = 0_u8;
        let mut last_value = 0_u8;
        let mut out_vec = std::vec::Vec::<u8>::with_capacity(encode_max_output_size(in_buf.len()));

        for x in in_buf {
            if run_len >= 0xFF {
                out_vec[code_i] = 0xFF;
                code_i = out_vec.len();
                run_len = 0;
            }
            if *x == 0 {
                if run_len == 0 {
                    out_vec.push(1);
                }
                else {
                    out_vec[code_i] = run_len;
                }
                code_i = out_vec.len();
                run_len = 0;
                last_value = 0;
            }
            else {
                if run_len == 0 {
                    out_vec.push(0xFF);
                    run_len = 1;
                }
                last_value = *x;
                out_vec.push(last_value);
                run_len += 1;
            }
        }

        // We've reached the end of the source data.
        // Finalise the remaining output. In particular, write the code (length) byte.
        if run_len == 0 {
            out_vec.push(1);
        }
        else {
            if last_value >= run_len {
                out_vec[code_i] = last_value;
                out_vec.pop();
            }
            else {
                out_vec[code_i] = run_len;
            }
        }

        Ok(out_vec)
    }

    pub fn decode_array<'a>(out_buf: &'a mut [u8], in_buf: &[u8]) -> crate::Result<&'a[u8]> {
        let mut code_i = 0;
        let mut out_i = 0;

        if in_buf.len() != 0 {
            loop {
                let code = in_buf[code_i];
                if code == 0 {
                    return Err(crate::Error::ZeroInEncodedData);
                }
                for in_i in (code_i + 1)..(code_i + code as usize) {
                    if out_i >= out_buf.len() {
                        return Err(crate::Error::OutputBufferTooSmall);
                    }
                    if in_i >= in_buf.len() {
                        // End of data, where length code is greater than remaining data.
                        // Output the length code as the last output byte.
                        out_buf[out_i] = code;
                        out_i += 1;
                        break;
                    }
                    let in_byte = in_buf[in_i];
                    if in_byte == 0 {
                        return Err(crate::Error::ZeroInEncodedData);
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

    pub fn decode_vector(in_buf: &[u8]) -> crate::Result<std::vec::Vec<u8>> {
        let mut code_i = 0;
        let mut out_vec = std::vec::Vec::<u8>::with_capacity(decode_max_output_size(in_buf.len()));

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
                        out_vec.push(code);
                        break;
                    }
                    let in_byte = in_buf[in_i];
                    if in_byte == 0 {
                        return Err(crate::Error::ZeroInEncodedData);
                    }
                    out_vec.push(in_byte);
                }
                code_i += code as usize;
                if code_i >= in_buf.len() {
                    // End of data. Exit, without outputting a trailing zero for the end of the data.
                    break;
                }
                if code < 0xFF {
                    // Output trailing zero.
                    out_vec.push(0);
                }
            }
        }
        Ok(out_vec)
    }
}
