//! This module contains functions for standard COBS encoding and decoding.

/// Calculate the minimum possible COBS encoded output size, for a given size of input data.
pub const fn encode_min_output_size(input_len: usize) -> usize {
    if input_len >= usize::max_value() - 1 {
        usize::max_value()
    } else {
        input_len + 1
    }
}

/// Calculate the maximum possible COBS encoded output size, for a given size of input data.
pub const fn encode_max_output_size(input_len: usize) -> usize {
    if input_len == 0 {
        1
    } else if input_len >= usize::max_value() - 253 {
        usize::max_value()
    } else {
        let increase = (input_len + 253) / 254;
        if input_len >= usize::max_value() - increase {
            usize::max_value()
        } else {
            input_len + increase
        }
    }
}

/// Common function for converting an iterator encoder's input iterator size hint to an output size hint.
fn encode_size_hint(in_hint: (usize, Option<usize>)) -> (usize, Option<usize>) {
    let lower_bound = encode_min_output_size(in_hint.0);
    let upper_bound = in_hint.1.map(|x| encode_max_output_size(x));
    (lower_bound, upper_bound)
}

/// Calculate the minimum possible decoded output size, for a given size of COBS-encoded input.
pub const fn decode_min_output_size(input_len: usize) -> usize {
    if input_len >= 1 {
        let increase = (input_len - 1) / 255;
        input_len - 1 - increase
    } else {
        0
    }
}

/// Calculate the maximum possible decoded output size, for a given size of COBS-encoded input.
pub const fn decode_max_output_size(input_len: usize) -> usize {
    if input_len > 1 {
        input_len - 1
    } else {
        0
    }
}

/// Common function for converting an iterator decoder's input iterator size hint to an output size hint.
fn decode_size_hint(in_hint: (usize, Option<usize>)) -> (usize, Option<usize>) {
    let lower_bound = decode_min_output_size(in_hint.0);
    let upper_bound = in_hint.1.map(|x| decode_max_output_size(x));
    (lower_bound, upper_bound)
}

/// Encode data into COBS encoded form, writing output to the given output buffer.
///
/// The output data is COBS-encoded, containing no zero-bytes.
///
/// The caller must provide a reference to a suitably-sized output buffer.
/// [encode_max_output_size()] calculates the required output buffer size, for a given input
/// size.
///
/// The return value is a [Result] that in the `Ok` case is a slice of the valid data in the
/// output buffer.
pub fn encode_array<'a>(out_buf: &'a mut [u8], in_buf: &[u8]) -> crate::Result<&'a [u8]> {
    let mut code_i = 0;
    let mut out_i = 1;

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
        } else {
            if out_i >= out_buf.len() {
                return Err(crate::Error::OutputBufferTooSmall);
            }
            out_buf[out_i] = *x;
            out_i += 1;
        }
    }

    // We've reached the end of the source data.
    // Finalise the remaining output. In particular, write the code (length) byte.
    // Update the pointer to calculate the final output length.
    out_buf[code_i] = (out_i - code_i) as u8;

    Ok(&out_buf[..out_i])
}

struct EncodeRefIterator<'a, I>
where
    I: Iterator<Item = &'a u8> + 'a,
{
    in_iter: I,
    eof: bool,
    last_run_0xff: bool,
    hold_write_i: u8,
    hold_read_i: u8,
    hold_buf: [u8; 255],
}

impl<'a, I> EncodeRefIterator<'a, I>
where
    I: Iterator<Item = &'a u8> + 'a,
{
    fn new(i: I) -> EncodeRefIterator<'a, I> {
        return EncodeRefIterator {
            in_iter: i,
            eof: false,
            last_run_0xff: false,
            hold_write_i: 0,
            hold_read_i: 0,
            hold_buf: [1; 255],
        };
    }
}

impl<'a, I> Iterator for EncodeRefIterator<'a, I>
where
    I: Iterator<Item = &'a u8> + 'a,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.hold_write_i != 0 {
            if self.hold_read_i < self.hold_write_i {
                let byte_val = self.hold_buf[self.hold_read_i as usize];
                self.hold_read_i += 1;
                return Some(byte_val);
            } else {
                self.hold_read_i = 0;
                self.hold_write_i = 0;
                // else drop through to loop below.
            }
        }
        if self.eof {
            return None;
        }
        loop {
            if self.hold_write_i == 0xFE {
                self.last_run_0xff = true;
                return Some(0xFF);
            } else {
                let in_iter_next = self.in_iter.next();
                let byte_ref = in_iter_next.unwrap_or_else(|| {
                    self.eof = true;
                    &0
                });
                if self.last_run_0xff {
                    self.last_run_0xff = false;
                    if self.eof {
                        return None;
                    }
                }
                if *byte_ref == 0 {
                    let count_byte = self.hold_write_i + 1;
                    return Some(count_byte);
                } else {
                    self.hold_buf[self.hold_write_i as usize] = *byte_ref;
                    self.hold_write_i += 1;
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let in_iter_size_hint = self.in_iter.size_hint();
        encode_size_hint(in_iter_size_hint)
    }
}

/// Encode data into COBS encoded form, getting data from a `&u8` iterator, and providing the output as an iterator.
///
/// The output data is COBS-encoded, containing no zero-bytes.
///
/// The caller must provide a `&u8` iterator.
///
/// The return value is a `u8` iterator. This is suitable to `collect()` into a byte container.
pub fn encode_ref_iter<'a, I>(i: I) -> impl Iterator<Item = u8> + 'a
where
    I: Iterator<Item = &'a u8> + 'a,
{
    EncodeRefIterator::<'a, I>::new(i)
}

struct EncodeIterator<I>
where
    I: Iterator<Item = u8>,
{
    in_iter: I,
    eof: bool,
    last_run_0xff: bool,
    hold_write_i: u8,
    hold_read_i: u8,
    hold_buf: [u8; 255],
}

impl<I> EncodeIterator<I>
where
    I: Iterator<Item = u8>,
{
    fn new(i: I) -> EncodeIterator<I> {
        return EncodeIterator {
            in_iter: i,
            eof: false,
            last_run_0xff: false,
            hold_write_i: 0,
            hold_read_i: 0,
            hold_buf: [1; 255],
        };
    }
}

impl<I> Iterator for EncodeIterator<I>
where
    I: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.hold_write_i != 0 {
            if self.hold_read_i < self.hold_write_i {
                let byte_val = self.hold_buf[self.hold_read_i as usize];
                self.hold_read_i += 1;
                return Some(byte_val);
            } else {
                self.hold_read_i = 0;
                self.hold_write_i = 0;
                // else drop through to loop below.
            }
        }
        if self.eof {
            return None;
        }
        loop {
            if self.hold_write_i == 0xFE {
                self.last_run_0xff = true;
                return Some(0xFF);
            } else {
                let in_iter_next = self.in_iter.next();
                let byte_val = in_iter_next.unwrap_or_else(|| {
                    self.eof = true;
                    0
                });
                if self.last_run_0xff {
                    self.last_run_0xff = false;
                    if self.eof {
                        return None;
                    }
                }
                if byte_val == 0 {
                    return Some(self.hold_write_i + 1);
                } else {
                    self.hold_buf[self.hold_write_i as usize] = byte_val;
                    self.hold_write_i += 1;
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let in_iter_size_hint = self.in_iter.size_hint();
        encode_size_hint(in_iter_size_hint)
    }
}

/// Encode data into COBS encoded form, getting data from a `u8` iterator, and providing the output as an iterator.
///
/// The output data is COBS-encoded, containing no zero-bytes.
///
/// The caller must provide a `u8` iterator.
///
/// The return value is a `u8` iterator. This is suitable to `collect()` into a byte container.
pub fn encode_iter(i: impl Iterator<Item = u8>) -> impl Iterator<Item = u8> {
    EncodeIterator::new(i)
}

/// Encode data into COBS encoded form, returning output as a vector of [u8].
///
/// The output data is COBS-encoded, containing no zero-bytes.
///
/// The return value is a [Result] that in the `Ok` case is a vector of [u8].
#[cfg(feature = "alloc")]
pub fn encode_vector(in_buf: &[u8]) -> crate::Result<alloc::vec::Vec<u8>> {
    let mut code_i = 0;
    let mut run_len = 0_u8;
    let mut out_vec = alloc::vec::Vec::<u8>::with_capacity(encode_max_output_size(in_buf.len()));

    for x in in_buf {
        if run_len == 0xFF {
            out_vec[code_i] = 0xFF;
            code_i += 0xFF;
            run_len = 0;
        }
        if *x == 0 {
            if run_len == 0 {
                out_vec.push(1);
                code_i += 1;
            } else {
                out_vec[code_i] = run_len;
                code_i += run_len as usize;
            }
            run_len = 0;
        } else {
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
    } else {
        out_vec[code_i] = run_len;
    }

    Ok(out_vec)
}

/// Decode COBS-encoded data, writing decoded data to the given output buffer.
///
/// The caller must provide a reference to a suitably-sized output buffer.
/// [decode_max_output_size()] calculates the required output buffer size, for a given input
/// size.
///
/// The return value is a [Result] that in the `Ok` case is a slice of the decoded data in the
/// output buffer.
pub fn decode_array<'a>(out_buf: &'a mut [u8], in_buf: &[u8]) -> crate::Result<&'a [u8]> {
    let mut code_i = 0;
    let mut out_i = 0;

    if !in_buf.is_empty() {
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

struct DecodeRefIterator<'a, I>
where
    I: Iterator<Item = &'a u8> + 'a,
{
    in_iter: I,
    eof: bool,
    last_run: u8,
    count_run: u8,
}

impl<'a, I> DecodeRefIterator<'a, I>
where
    I: Iterator<Item = &'a u8> + 'a,
{
    fn new(i: I) -> DecodeRefIterator<'a, I> {
        return DecodeRefIterator {
            in_iter: i,
            eof: false,
            last_run: 0,
            count_run: 0,
        };
    }
}

impl<'a, I> Iterator for DecodeRefIterator<'a, I>
where
    I: Iterator<Item = &'a u8> + 'a,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let in_iter_next = self.in_iter.next();
            let byte_val = in_iter_next.map(|x| *x).unwrap_or(0);
            if byte_val == 0 {
                return None;
            }
            if self.count_run == 0 {
                let last_run = self.last_run;
                self.last_run = byte_val;
                self.count_run = byte_val - 1;
                if last_run != 0 && last_run != 0xFF {
                    return Some(0);
                }
            } else {
                self.count_run -= 1;
                return Some(byte_val);
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let in_iter_size_hint = self.in_iter.size_hint();
        decode_size_hint(in_iter_size_hint)
    }
}

/// Decode COBS-encoded data, getting data from a `&u8` iterator, and providing the output as an iterator.
///
/// The input data should be COBS-encoded, containing no zero-bytes.
///
/// The caller must provide a `&u8` iterator.
///
/// The return value is a `u8` iterator. This is suitable to `collect()` into a byte container.
///
/// Unlike the other decode functions, no errors are returned by this function. Rather, decoding is
/// best-effort. In the event of any zero in the input, this will be regarded as end-of-data. If
/// insufficient bytes are present following a length code, the output will simply stop at the end
/// of the available data.
pub fn decode_ref_iter<'a, I>(i: I) -> impl Iterator<Item = u8> + 'a
where
    I: Iterator<Item = &'a u8> + 'a,
{
    DecodeRefIterator::<'a, I>::new(i)
}

struct DecodeIterator<I>
where
    I: Iterator<Item = u8>,
{
    in_iter: I,
    eof: bool,
    last_run: u8,
    count_run: u8,
}

impl<I> DecodeIterator<I>
where
    I: Iterator<Item = u8>,
{
    fn new(i: I) -> DecodeIterator<I> {
        return DecodeIterator {
            in_iter: i,
            eof: false,
            last_run: 0,
            count_run: 0,
        };
    }
}

impl<I> Iterator for DecodeIterator<I>
where
    I: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let in_iter_next = self.in_iter.next();
            let byte_val = in_iter_next.unwrap_or(0);
            if byte_val == 0 {
                return None;
            }
            if self.count_run == 0 {
                let last_run = self.last_run;
                self.last_run = byte_val;
                self.count_run = byte_val - 1;
                if last_run != 0 && last_run != 0xFF {
                    return Some(0);
                }
            } else {
                self.count_run -= 1;
                return Some(byte_val);
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let in_iter_size_hint = self.in_iter.size_hint();
        decode_size_hint(in_iter_size_hint)
    }
}

/// Decode COBS-encoded data, getting data from a `u8` iterator, and providing the output as an iterator.
///
/// The input data should be COBS-encoded, containing no zero-bytes.
///
/// The caller must provide a `u8` iterator.
///
/// The return value is a `u8` iterator. This is suitable to `collect()` into a byte container.
///
/// Unlike the other decode functions, no errors are returned by this function. Rather, decoding is
/// best-effort. In the event of any zero in the input, this will be regarded as end-of-data. If
/// insufficient bytes are present following a length code, the output will simply stop at the end
/// of the available data.
pub fn decode_iter<I>(i: I) -> impl Iterator<Item = u8>
where
    I: Iterator<Item = u8>,
{
    DecodeIterator::<I>::new(i)
}

/// Decode COBS-encoded data, returning output as a vector of [u8].
///
/// The return value is a [Result] that in the `Ok` case is a vector of [u8].
#[cfg(feature = "alloc")]
pub fn decode_vector(in_buf: &[u8]) -> crate::Result<alloc::vec::Vec<u8>> {
    let mut code_i = 0;
    let mut out_vec = alloc::vec::Vec::<u8>::with_capacity(decode_max_output_size(in_buf.len()));

    if !in_buf.is_empty() {
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